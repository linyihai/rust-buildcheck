use super::build_rule::BuildRule;
use super::build_rule::Detail;
use anyhow::Result;
use cargo_metadata::Error as MetaDataError;
use git2::Error as Git2Error;

pub type CheckResult<T> = Result<T, CheckError>;

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

pub fn build_detail_err(
    build_rule: BuildRule,
    location: String,
    description: String,
    line: usize,
) -> CheckResult<()> {
    Err(CheckError::CheckDetail(Detail::build(
        build_rule,
        location,
        description,
        line,
    )))
}
