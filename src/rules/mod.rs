mod cargo_check;
mod errors;
mod metadata_util;
mod package_check;

use crate::commands::Args;
use cargo_check::LockCheck;
use cargo_metadata::{CargoOpt, Error as MetaDataError, Metadata, MetadataCommand};
use errors::{BuildError, BuildResult};
use package_check::PackageCheck;

trait BuildRule {
    fn check(&self, m: &Metadata, args: &Args) -> Vec<BuildResult<()>>;
}

fn get_metadata(manifesh_path: &String) -> Result<Metadata, MetaDataError> {
    MetadataCommand::new()
        .no_deps()
        .features(CargoOpt::AllFeatures)
        .manifest_path(manifesh_path)
        .exec()
}

pub struct BuildRuleChecker {
    rules: Vec<Box<dyn BuildRule>>,
    metadata: Metadata,
    args: Args,
}

impl BuildRuleChecker {
    pub fn new(args: Args) -> BuildResult<BuildRuleChecker> {
        Ok(BuildRuleChecker {
            rules: vec![Box::new(LockCheck::new()), Box::new(PackageCheck::new())],
            metadata: get_metadata(&args.manifest_path).map_err(BuildError::MetaData)?,
            args,
        })
    }

    pub fn check_rule(&self) {
        let mut check_res: Vec<Result<(), BuildError>> = vec![];
        for checker in &self.rules {
            let mut res = checker.check(&self.metadata, &self.args);
            check_res.append(&mut res);
        }
        let check_failed_err = check_res
            .iter()
            .filter_map(|t| if let Err(err) = t { Some(err) } else { None })
            .collect::<Vec<&BuildError>>();
        for err in &check_failed_err {
            println!("{}", err);
        }
        if !check_failed_err.is_empty() {
            std::process::exit(1)
        } else {
            println!("All build check done.");
        }
    }
}
