use super::RuleChecker;
use crate::commands::Args;
use crate::errors::{build_detail_err, BuildRule, CheckError, CheckResult};
use cargo_metadata::{Metadata, Package};

#[derive(Debug, Clone, Default)]
pub struct LockCheck;

impl LockCheck {
    // check Cargo.toml whether is commit
    fn check_file(&self, package: &Package, repo: &git2::Repository) -> CheckResult<()> {
        let parent_path = package.manifest_path.parent().unwrap().as_std_path();
        let repo_root = repo.workdir().unwrap();

        let binding = parent_path
            .strip_prefix(repo_root)
            .unwrap()
            .join("Cargo.toml");
        let toml_file = binding.as_path();
        let status = repo.status_file(toml_file)?;
        if status.is_index_new() || status.is_wt_new() {
            return build_detail_err(
                BuildRule::GRS08,
                toml_file.display().to_string(),
                format!("`{}` was not committed in package.", toml_file.display()),
            );
        }
        Ok(())
    }
}

impl RuleChecker for LockCheck {
    fn check(&self, m: &Metadata, _: &Args) -> Vec<CheckResult<()>> {
        let root_package = m.root_package();
        let mut check_res = vec![];
        if let Some(root_package) = root_package {
            if let Some(root_path) = root_package.manifest_path.parent() {
                let res = git2::Repository::discover(root_path);
                if let Err(err) = res {
                    check_res.push(Err(CheckError::Git2(err)));
                    return check_res;
                }
                for package in &m.packages {
                    println!("package name: {}", package.name);
                    if let Ok(repo) = &res {
                        check_res.push(self.check_file(package, repo));
                    }
                }
            }
        }
        check_res
    }
}
