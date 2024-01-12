use std::fs;

use super::RuleChecker;
use crate::utils::{
    build_rule::BuildRule,
    commands::{cargo::cargo_package, Args},
    custom_error::{build_detail_err, CheckResult},
    toml::{cargo_toml::read_toml_file, display_line},
};
use anyhow::Context;
use cargo_metadata::{Metadata, Package};

static MB: f32 = 1000000.0;
#[derive(Debug, Clone, Default)]
pub struct PackageCheck;

impl PackageCheck {
    // check crate size whether over 10MB
    fn check_package_size(&self, package: &Package, max_size: f32) -> CheckResult<()> {
        let create_name = format!("{}-{}.crate", package.name, package.version);
        let dir = package.manifest_path.parent().unwrap().as_std_path();
        let crate_path = dir.join("target").join("package").join(&create_name);
        cargo_package(package.manifest_path.as_std_path())?;
        let crate_size = fs::metadata(crate_path)
            .with_context(|| "cannot find the packed crate")?
            .len();
        let max_crate_size = max_size * MB;
        if crate_size as f32 > max_crate_size {
            return build_detail_err(
                BuildRule::GRS20,
                dir.display().to_string(),
                format!("package `{}` size over {}MB.", &create_name, max_size),
                0,
            );
        }
        Ok(())
    }

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
    fn check(&self, m: &Metadata, args: &Args) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];

        for package in &m.packages {
            check_res.push(self.check_package_name(package));
            check_res.push(self.check_package_size(package, args.crate_size));
        }
        check_res
    }
}

fn is_valid_package_name(name: &str) -> bool {
    ["ylong_", "huawei_"].iter().any(|p| name.starts_with(p))
}
