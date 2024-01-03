use std::fs;

use super::RuleChecker;
use crate::commands::pack_crate;
use crate::commands::Args;
use crate::errors::{build_detail_err, BuildRule, CheckResult};
use anyhow::Context;
use cargo_metadata::Metadata;

#[derive(Debug, Clone, Default)]
pub struct PackageCheck;

impl PackageCheck {
    // check crate size whether over 10MB
    fn check_package_size(&self, m: &Metadata, max_size: f32) -> CheckResult<()> {
        let root_package = m.root_package().unwrap();
        let create_name = format!("{}-{}.crate", root_package.name, root_package.version);
        let dir = root_package.manifest_path.parent().unwrap().as_std_path();
        let crate_path = dir.join("target").join("package").join(&create_name);
        pack_crate(root_package.manifest_path.as_std_path())?;
        let crate_size = fs::metadata(crate_path)
            .with_context(|| "cannot find the packed crate")?
            .len();
        let max_crate_size = max_size * 1000000.0;
        if crate_size as f32 > max_crate_size {
            return build_detail_err(
                BuildRule::GRS20,
                dir.display().to_string(),
                format!("crate `{}` size over {}MB.", &create_name, max_size),
            );
        }
        Ok(())
    }

    fn check_package_name(&self, m: &Metadata) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];

        for package in &m.packages {
            if !is_valid_package_name(&package.name) {
                check_res.push(build_detail_err(
                    BuildRule::GRS17,
                    package.manifest_path.as_str().to_string(),
                    format!(
                        "crate name `{}` not start with ylong_ or huawei_ .",
                        package.name
                    ),
                ));
            }
        }
        check_res
    }
}

impl RuleChecker for PackageCheck {
    fn check(&self, m: &Metadata, args: &Args) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];
        let res = self.check_package_size(m, args.crate_size);
        if res.is_err() {
            check_res.push(res);
        }

        check_res.append(&mut self.check_package_name(m));
        check_res
    }
}

fn is_valid_package_name(name: &str) -> bool {
    ["ylong_", "huawei_"].iter().any(|p| name.starts_with(p))
}
