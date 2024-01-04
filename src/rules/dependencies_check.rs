use super::RuleChecker;
use crate::utils::{
    build_rule::BuildRule,
    commands::Args,
    custom_error::{build_detail_err, CheckResult},
    toml::{cargo_toml::read_toml_file, display_line},
};
use cargo_metadata::DependencyKind;
use cargo_metadata::Metadata;
use if_chain::if_chain;

#[derive(Debug, Clone, Default)]
pub struct DependenciesCheck;

impl RuleChecker for DependenciesCheck {
    fn check(&self, m: &Metadata, _: &Args) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];
        for package in &m.packages {
            for dep in &package.dependencies {
                if_chain! {
                    if let Some(ref source) = dep.source;
                    if !source.starts_with("git");
                    if !is_explicit_version(&dep.req);
                    then {
                        let (toml_config, contents) = read_toml_file(package.manifest_path.as_std_path());
                        let (toml_dependencies, section) = match dep.kind {
                            DependencyKind::Normal if dep.target.is_none() => (&toml_config.dependencies, ""),
                            DependencyKind::Build if dep.target.is_none() => (&toml_config.build_dependencies, "build-"),
                            DependencyKind::Development if dep.target.is_none() => (&toml_config.dev_dependencies, "dev-"),
                            _ => (&None, "")
                        };

                        let detail = {
                            if dep.target.is_some() {
                                format!(
                                    "package `{}` use a inexplicit version dependency `{}` with target `{}`, please replace it with a explicit version",
                                    package.name, dep.name, dep.target.as_ref().unwrap()
                                )
                            } else {
                                format!(
                                    "package `{}` use a inexplicit version {}dependency `{}`, please replace it with a explicit version",
                                    package.name, section, dep.name
                                )
                            }
                        };

                        let line = if_chain! {
                            if let Some(dependencies) = toml_dependencies;
                            if let Some(v) = dependencies.get(&dep.name);
                            then {
                                display_line(contents.as_bytes(), v.span().start)
                            } else {
                                0
                            }
                        };

                        let err = build_detail_err(BuildRule::GRS18, package.manifest_path.as_str().to_string(),  detail, line);
                        check_res.push(err);
                    }
                }
            }
        }
        check_res
    }
}

fn is_explicit_version(req: &semver::VersionReq) -> bool {
    if req.comparators.len() != 1 {
        return false;
    }

    let major = req.comparators[0].major;
    let minor = req.comparators[0].minor.unwrap_or_default();
    let patch = req.comparators[0].patch.unwrap_or_default();

    [
        format!("{major}.{minor}.{patch}"),
        format!("={major}.{minor}.{patch}"),
    ]
    .iter()
    .map(|ver| {
        if let Ok(expect_ver) = semver::VersionReq::parse(ver) {
            return &expect_ver == req;
        }
        false
    })
    .any(|t| t)
}
