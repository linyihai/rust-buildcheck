use super::RuleChecker;
use crate::utils::{
    build_rule::BuildRule,
    commands::Args,
    custom_error::{build_detail_err, CheckError, CheckResult},
};
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
        if is_no_commit(status) {
            return build_detail_err(
                BuildRule::GRS08,
                parent_path.join("Cargo.toml").display().to_string(),
                format!("`{}` was not committed in package.", toml_file.display()),
                0,
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
                    if let Ok(repo) = &res {
                        check_res.push(self.check_file(package, repo));
                    }
                }
            }
        }
        check_res
    }
}

fn is_no_commit(status: git2::Status) -> bool {
    status.is_index_new()
        || status.is_index_modified()
        || status.is_index_deleted()
        || status.is_index_renamed()
        || status.is_index_typechange()
        || status.is_wt_new()
        || status.is_wt_modified()
        || status.is_wt_deleted()
        || status.is_wt_typechange()
        || status.is_wt_renamed()
        || status.is_ignored()
        || status.is_conflicted()
}
