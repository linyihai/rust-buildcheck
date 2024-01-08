use super::RuleChecker;
use crate::utils::{
    build_rule::BuildRule,
    commands::{pack_crate, Args},
    custom_error::{build_detail_err, CheckResult},
    toml::{cargo_toml::read_toml_file, display_line},
};
use anyhow::Context;
use cargo_metadata::Metadata;
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct PackageCheck;

impl PackageCheck {
    // check crate size whether over 10MB
    fn check_package_size(&self, args: &Args, m: &Metadata, max_size: f32) -> CheckResult<()> {
        let root_package = m.root_package().unwrap();
        let create_name = format!("{}-{}.crate", root_package.name, root_package.version);
        let dir = root_package.manifest_path.parent().unwrap().as_std_path();
        let crate_path = dir.join("target").join("package").join(&create_name);
        pack_crate(args, root_package.manifest_path.as_std_path())?;
        let crate_size = fs::metadata(crate_path)
            .with_context(|| "cannot find the packed crate")?
            .len();
        let max_crate_size = max_size * 1000000.0;
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

    fn check_package_name(&self, m: &Metadata) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];

        for package in &m.packages {
            if !is_valid_package_name(&package.name) {
                let (toml_config, contents) = read_toml_file(package.manifest_path.as_std_path());
                let line = if let Some(name) = toml_config.package.name {
                    let start = name.span().start;
                    display_line(contents.as_bytes(), start)
                } else {
                    0
                };

                check_res.push(build_detail_err(
                    BuildRule::GRS17,
                    package.manifest_path.as_str().to_string(),
                    format!(
                        "package name `{}` not start with ylong_ or huawei_ .",
                        package.name
                    ),
                    line,
                ));
            }
        }
        check_res
    }
}

impl RuleChecker for PackageCheck {
    fn check(&self, m: &Metadata, args: &Args) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];
        let res = self.check_package_size(args, m, args.crate_size);
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
