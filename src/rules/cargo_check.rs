// G.RS.08
// G.RS.14
use std::fs;

use super::errors::{CheckError, CheckResult};
use super::metadata_util::{self, TargetType};
use super::RuleChecker;
use crate::commands::Args;
use cargo_metadata::{Metadata, Package};

#[derive(Debug, Clone, Default)]
pub struct LockCheck;

impl LockCheck {
    // check Cargo.lock commit in binary package, and not commit in library package
    // check Cargo.toml whether is commit
    fn check_file(&self, package: &Package, repo: &git2::Repository) -> CheckResult<()> {
        let target_type = metadata_util::get_target_type(package);
        let parent_path = package.manifest_path.parent().unwrap().as_std_path();
        let repo_root = repo.workdir().unwrap();
        let binding = parent_path
            .strip_prefix(repo_root)
            .unwrap()
            .join("Cargo.lock");
        let lock_file = binding.as_path();
        if fs::metadata(lock_file).is_ok() {
            let status = repo.status_file(lock_file)?;
            if target_type == TargetType::Binary && (status.is_index_new() || status.is_wt_new()) {
                return Err(CheckError::Check {
                    stderr: format!(
                        "`[G.RS.14] {}` was not committed in binary package.",
                        lock_file.display()
                    ),
                });
            }
            if target_type == TargetType::Libaray && !(status.is_index_new() || status.is_wt_new())
            {
                return Err(CheckError::Check {
                    stderr: format!(
                        "`[G.RS.14] {}` was committed in library package.",
                        lock_file.display()
                    ),
                });
            }
        } else if target_type == TargetType::Binary {
            return Err(CheckError::Check {
                stderr: format!(
                    "`[G.RS.14] {}` was not committed in binary package.",
                    lock_file.display()
                ),
            });
        }

        let binding = parent_path
            .strip_prefix(repo_root)
            .unwrap()
            .join("Cargo.toml");
        let toml_file = binding.as_path();
        let status = repo.status_file(toml_file)?;
        if status.is_index_new() || status.is_wt_new() {
            return Err(CheckError::Check {
                stderr: format!(
                    "[G.RS.08] {} not committed in package.",
                    toml_file.display()
                ),
            });
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
