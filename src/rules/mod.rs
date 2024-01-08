mod cargo_check;
mod dependencies_check;
mod license_check;
mod package_check;
mod rust_edition_check;

use crate::rules::license_check::LicenseCheck;
use crate::utils::{
    commands::Args,
    custom_error::{CheckError, CheckResult},
    output::Output,
};
use anyhow::{Context, Result};
use cargo_check::LockCheck;
use cargo_metadata::{CargoOpt, Metadata, MetadataCommand};
use dependencies_check::DependenciesCheck;
use package_check::PackageCheck;
use rust_edition_check::EditionCheck;
use std::fs::{self, File};

trait RuleChecker {
    fn check(&self, m: &Metadata, args: &Args) -> Vec<CheckResult<()>>;
}

pub struct BuildRuleChecker {
    rules: Vec<Box<dyn RuleChecker>>,
    metadata: Metadata,
    args: Args,
}

impl BuildRuleChecker {
    pub fn new(args: Args) -> CheckResult<BuildRuleChecker> {
        _ = File::create(&args.output_file).with_context(|| "output_file path invalid")?;
        fs::metadata(&args.manifest_path)
            .with_context(|| "please check you -m flag, the Cargo.toml path may be invalid")?;

        let mut binding = MetadataCommand::new();
        binding
            .no_deps()
            .features(CargoOpt::AllFeatures)
            .manifest_path(&args.manifest_path);

        if args.offline {
            binding.other_options(vec!["--offline".to_string()]);
        }

        let metadata = binding.exec().with_context(|| "get cargo metadata failed, check if cargo is installed and the contents of Cargo.toml are correct")?;

        Ok(BuildRuleChecker {
            rules: vec![
                Box::new(LockCheck),
                Box::new(PackageCheck),
                Box::new(EditionCheck),
                Box::new(DependenciesCheck),
                Box::new(LicenseCheck),
            ],
            metadata,
            args,
        })
    }

    pub fn check_rule(&self) -> Result<()> {
        let mut check_res: Vec<Result<(), CheckError>> = vec![];
        for checker in &self.rules {
            let mut res = checker.check(&self.metadata, &self.args);
            check_res.append(&mut res);
        }
        let check_failed_err = check_res
            .iter()
            .filter_map(|t| if let Err(err) = t { Some(err) } else { None })
            .collect::<Vec<&CheckError>>();
        for err in &check_failed_err {
            if let CheckError::CheckDetail(detail) = err {
                print!("{}", detail);
            }
        }

        let is_check_ok = &check_failed_err.is_empty();
        println!("the output file is {}", &self.args.output_file);
        let output = serde_json::to_string(&Output::from(check_failed_err)).unwrap();
        fs::write(&self.args.output_file, output)
            .with_context(|| "save check result to file failed")?;
        if !is_check_ok {
            std::process::exit(1)
        } else {
            println!("All build check done.");
        }
        Ok(())
    }
}
