use std::fmt::{self, Display};

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
    pub errno: u32,
    pub check_type: String,
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
    GRS17,
    GRS18,
    GRS20,
}

impl fmt::Display for BuildRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BuildRule::GRS05 => write!(f, "[G.RS.05]"),
            BuildRule::GRS06 => write!(f, "[G.RS.06]"),
            BuildRule::GRS08 => write!(f, "[G.RS.08]"),
            BuildRule::GRS10 => write!(f, "[G.RS.10]"),
            BuildRule::GRS17 => write!(f, "[G.RS.17]"),
            BuildRule::GRS18 => write!(f, "[G.RS.18]"),
            BuildRule::GRS20 => write!(f, "[G.RS.20]"),
        }
    }
}

impl From<BuildRule> for Detail {
    fn from(value: BuildRule) -> Self {
        match value {
            BuildRule::GRS05 => Detail {
                level: Level::Rule,
                errno: 31004_u32,
                check_type: String:: from("build tool"),
                ..Detail::default()
            },
            BuildRule::GRS06 => Detail {
                level: Level::Suggestion,
                errno: 31005_u32,
                check_type: String:: from("build tool"),
                ..Detail::default()
            },
            BuildRule::GRS08 => Detail {
                level: Level::Rule,
                errno: 31007_u32,
                check_type: String:: from("build configuration"),
                ..Detail::default()
            },
            BuildRule::GRS10 => Detail {
                level: Level::Rule,
                errno: 31009_u32,
                check_type: String:: from("build configuration"),
                ..Detail::default()
            },
            BuildRule::GRS17 => Detail {
                level: Level::Rule,
                errno: 31016_u32,
                check_type: String:: from("packaging and pushlishing"),
                ..Detail::default()
            },
            BuildRule::GRS18 => Detail {
                level: Level::Rule,
                errno: 31017_u32,
                check_type: String:: from("packaging and pushlishing"),
                ..Detail::default()
            },
            BuildRule::GRS20 => Detail {
                level: Level::Rule,
                errno: 31019_u32,
                check_type: String:: from("packaging and pushlishing"),
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
