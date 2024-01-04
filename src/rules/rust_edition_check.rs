use super::RuleChecker;
use crate::utils::{
    build_rule::BuildRule,
    commands::Args,
    custom_error::{build_detail_err, CheckResult},
    toml::{cargo_toml::read_toml_file, display_line},
};
use cargo_metadata::{Edition, Metadata};

#[derive(Debug, Clone, Default)]
pub struct EditionCheck;

impl RuleChecker for EditionCheck {
    fn check(&self, m: &Metadata, _: &Args) -> Vec<CheckResult<()>> {
        let mut check_res = vec![];
        for package in &m.packages {
            let (toml_config, contents) = read_toml_file(package.manifest_path.as_std_path());
            let (line, edition) = if let Some(edition) = toml_config.package.edition {
                let start = edition.span().start;
                let line = display_line(contents.as_bytes(), start);
                (line, Some(edition))
            } else {
                (0, None)
            };

            if package.edition == Edition::E2015 && edition.is_none() {
                let detail =
                    "no `edition` field in Cargo.toml, please add a edition like `edition = 2021`.";
                check_res.push(build_detail_err(
                    BuildRule::GRS05,
                    package.manifest_path.as_str().to_string(),
                    detail.to_string(),
                    line,
                ));
                continue;
            }

            if package.edition < Edition::E2021 {
                let detail = "the `edition` field in Cargo.toml is outdated, please update it with new edition, like `2021`";
                check_res.push(build_detail_err(
                    BuildRule::GRS06,
                    package.manifest_path.as_str().to_string(),
                    detail.to_string(),
                    line,
                ));
            }
        }
        check_res
    }
}
