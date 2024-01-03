use crate::rules::RuleChecker;
use crate::utils::{
    build_rule::BuildRule,
    commands::Args,
    custom_error::{build_detail_err, CheckResult},
    toml::{cargo_toml::read_toml_file, display_line},
};
use cargo_metadata::Metadata;

#[derive(Debug, Clone, Default)]
pub struct LicenseCheck;

impl RuleChecker for LicenseCheck {
    fn check(&self, m: &Metadata, _args: &Args) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];
        for package in &m.packages {
            if package.license.is_none() && package.license_file().is_none() {
                continue;
            }
            let (toml_config, contents) = read_toml_file(package.manifest_path.as_std_path());
            if package.license.is_some() {
                let line = if let Some(license) = toml_config.package.license {
                    let start = license.span().start;
                    display_line(contents.as_bytes(), start)
                } else {
                    0
                };

                let err = build_detail_err(
                    BuildRule::GRS10,
                    package.manifest_path.as_str().to_string(),
                    format!("package `{}` contains `license` field in Cargo.toml.", package.name),
                    line,
                );
                check_res.push(err)
            }
            if package.license_file.is_some() {
                let line = if let Some(license_file) = toml_config.package.license_file {
                    let start = license_file.span().start;
                    display_line(contents.as_bytes(), start)
                } else {
                    0
                };
                let err = build_detail_err(
                    BuildRule::GRS10,
                    package.manifest_path.as_str().to_string(),
                    format!("package `{}` contains `license-file` field in Cargo.toml.", package.name),
                    line,
                );
                check_res.push(err)
            }
        }
        check_res
    }
}
