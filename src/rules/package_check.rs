use super::RuleChecker;
use crate::utils::{
    build_rule::BuildRule,
    commands::Args,
    custom_error::{build_detail_err, CheckResult},
    toml::{cargo_toml::read_toml_file, display_line},
};
use cargo_metadata::{Metadata, Package};

#[derive(Debug, Clone, Default)]
pub struct PackageCheck;

impl PackageCheck {
    fn check_package_name(&self, package: &Package) -> CheckResult<()> {
        if !is_valid_package_name(&package.name) {
            let (toml_config, contents) = read_toml_file(package.manifest_path.as_std_path());
            let line = if let Some(name) = toml_config.package.name {
                let start = name.span().start;
                display_line(contents.as_bytes(), start)
            } else {
                0
            };

            return build_detail_err(
                BuildRule::GRS17,
                package.manifest_path.as_str().to_string(),
                format!(
                    "package name `{}` not start with ylong_ or huawei_ .",
                    package.name
                ),
                line,
            );
        }

        Ok(())
    }
}

impl RuleChecker for PackageCheck {
    fn check(&self, m: &Metadata, _: &Args) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];

        for package in &m.packages {
            check_res.push(self.check_package_name(package));
        }
        check_res
    }
}

fn is_valid_package_name(name: &str) -> bool {
    ["ylong_", "huawei_"].iter().any(|p| name.starts_with(p))
}
