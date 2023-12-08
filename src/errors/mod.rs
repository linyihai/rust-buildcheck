use std::fmt::Display;

use crate::output::{self, Level};
use anyhow::Result;
use cargo_metadata::Error as MetaDataError;
use git2::Error as Git2Error;

pub type CheckResult<T> = Result<T, CheckError>;
// custom build check error which contains MetadataError
#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    #[error("build check failed: {0}")]
    CheckDetail(Detail),
    #[error("{0}")]
    MetaData(#[from] MetaDataError),
    #[error("{0}")]
    Git2(#[from] Git2Error),
    #[error("{0}")]
    AnyHow(#[from] anyhow::Error),
}

#[derive(Debug, Default)]
pub struct Detail {
    pub description: String,
    pub level: output::Level,
    pub location: String,
}

impl Display for Detail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.description)
    }
}

pub enum BuildRule {
    GRS05,
    GRS06,
    GRS08,
    GRS10,
    GRS14,
    GRS18,
    GRS19,
    GRS21,
}

impl From<BuildRule> for Detail {
    fn from(value: BuildRule) -> Self {
        match value {
            BuildRule::GRS06 => Detail {
                level: Level::Suggestion,
                ..Detail::default()
            },
            _ => Detail {
                level: Level::Rule,
                ..Detail::default()
            },
        }
    }
}

impl Detail {
    pub fn build(build_rule: BuildRule, location: String, description: String) -> Self {
        Self {
            location,
            description,
            ..Detail::from(build_rule)
        }
    }
}

pub fn build_detail_err(
    build_rule: BuildRule,
    location: String,
    description: String,
) -> CheckResult<()> {
    Err(CheckError::CheckDetail(Detail::build(
        build_rule,
        location,
        description,
    )))
}
