mod cargo_check;
mod dependencies_check;
mod metadata_util;
mod package_check;
mod rust_edition_check;

use std::fs::{self, File};

use crate::commands::Args;
use crate::errors::{CheckError, CheckResult};
use crate::output;
use anyhow::anyhow;
use cargo_check::LockCheck;
use cargo_metadata::{CargoOpt, Error as MetaDataError, Metadata, MetadataCommand};
use dependencies_check::DependenciesCheck;
use package_check::PackageCheck;
use rust_edition_check::EditionCheck;

trait RuleChecker {
    fn check(&self, m: &Metadata, args: &Args) -> Vec<CheckResult<()>>;
}

fn get_metadata(manifesh_path: &String) -> Result<Metadata, MetaDataError> {
    MetadataCommand::new()
        .no_deps()
        .features(CargoOpt::AllFeatures)
        .manifest_path(manifesh_path)
        .exec()
}

pub struct BuildRuleChecker {
    rules: Vec<Box<dyn RuleChecker>>,
    metadata: Metadata,
    args: Args,
}

impl BuildRuleChecker {
    pub fn new(args: Args) -> CheckResult<BuildRuleChecker> {
        if File::create(&args.output_file).is_err() {
            return Err(anyhow!("output_file path invalid").into());
        }
        Ok(BuildRuleChecker {
            rules: vec![
                Box::new(LockCheck),
                Box::new(PackageCheck),
                Box::new(EditionCheck),
                Box::new(DependenciesCheck),
            ],
            metadata: get_metadata(&args.manifest_path).map_err(CheckError::MetaData)?,
            args,
        })
    }

    pub fn check_rule(&self) {
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
            println!("{}", err);
        }

        let is_check_ok = &check_failed_err.is_empty();
        println!("the output file is {}", &self.args.output_file);
        let output = serde_json::to_string(&output::Output::from(check_failed_err)).unwrap();
        fs::write(&self.args.output_file, output).unwrap();
        if !is_check_ok {
            std::process::exit(1)
        } else {
            println!("All build check done.");
        }
    }
}
