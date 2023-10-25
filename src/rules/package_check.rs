use std::fs;

use super::errors::{BuildError, BuildResult};
use super::BuildRule;
use crate::commands::pack_crate;
use crate::commands::Args;
use cargo_metadata::Metadata;

#[derive(Debug, Clone, Default)]
pub struct PackageCheck;

impl PackageCheck {
    pub fn new() -> PackageCheck {
        PackageCheck
    }
}

impl PackageCheck {
    // check crate size whether over 10MB
    fn check_package_size(&self, m: &Metadata, max_size: f32) -> BuildResult<()> {
        let root_package = m.root_package().unwrap();
        let create_name = format!("{}-{}.crate", root_package.name, root_package.version);
        let dir = root_package.manifest_path.parent().unwrap().as_std_path();
        let crate_path = dir.join("target").join("package").join(&create_name);
        pack_crate(root_package.manifest_path.as_std_path());
        let crate_size = fs::metadata(crate_path).unwrap().len();
        let max_crate_size = max_size * 1000000.0;
        if crate_size as f32 > max_crate_size {
            return Err(BuildError::Check {
                stderr: format!("crate `{}` size over {}MB.", &create_name, max_size),
            });
        }
        Ok(())
    }
}

impl BuildRule for PackageCheck {
    fn check(&self, m: &Metadata, args: &Args) -> Vec<BuildResult<()>> {
        vec![(self.check_package_size(m, args.crate_size))]
    }
}
