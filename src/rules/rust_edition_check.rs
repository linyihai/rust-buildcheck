use super::errors::{CheckError, CheckResult};
use super::RuleChecker;
use crate::commands::Args;
use cargo_metadata::{Edition, Metadata};
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct TomlConfig {
    package: PackageConfig,
}

#[derive(Deserialize, Debug)]
struct PackageConfig {
    edition: Option<Edition>,
}

fn read_toml_file(path: &Path) -> TomlConfig {
    let contents = fs::read_to_string(path).unwrap();
    toml::from_str(&contents).unwrap()
}

#[derive(Debug, Clone, Default)]
pub struct EditionCheck;

impl RuleChecker for EditionCheck {
    fn check(&self, m: &Metadata, _: &Args) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];
        for package in &m.packages {
            if package.edition == Edition::E2015 {
                let toml_config = read_toml_file(package.manifest_path.as_std_path());
                if toml_config.package.edition.is_none() {
                    check_res.push(Err(CheckError::Check {
                        stderr: format!(
                        "[G.RS.05] {} has no edition field, please add a edition like `edition = 2021`.",
                        package.manifest_path
                    ),
                    }));
                }
            } 
            if package.edition < Edition::E2021 {
                check_res.push(Err(CheckError::Check {
                    stderr: format!(
                        "[G.RS.06] {} no newest rust editon.",
                        package.manifest_path
                    ),
                }));
            }
        }

        check_res
    }
}
