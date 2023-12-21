use super::RuleChecker;
use crate::commands::Args;
use crate::errors::{build_detail_err, BuildRule, CheckResult};
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
                        let err = build_detail_err(BuildRule::GRS18, "".to_string(),  format!(
                            "package {} use no explicit version for dependency {}.",
                            package.name, dep.name
                        ));
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
