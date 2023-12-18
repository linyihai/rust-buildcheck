// G.RS.10
use crate::commands::Args;
use crate::errors::{build_detail_err, BuildRule, CheckResult};
use crate::rules::RuleChecker;
use cargo_metadata::Metadata;

#[derive(Debug, Clone, Default)]
pub struct LicenseCheck;

impl RuleChecker for LicenseCheck {
    fn check(&self, m: &Metadata, _args: &Args) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];
        for package in &m.packages {
            if package.license.is_some() {
                let err = build_detail_err(
                    BuildRule::GRS10,
                    "".to_string(),
                    format!(
                        "[G.RS.10] package `{}` contains `license` field",
                        package.name
                    ),
                );
                check_res.push(err)
            }
            if package.license_file.is_some() {
                let err = build_detail_err(
                    BuildRule::GRS10,
                    "".to_string(),
                    format!(
                        "[G.RS.10] package `{}` contains `license-file` field",
                        package.name
                    ),
                );
                check_res.push(err)
            }
        }
        check_res
    }
}
