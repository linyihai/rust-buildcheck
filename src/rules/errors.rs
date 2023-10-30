use cargo_metadata::Error as MetaDataError;
use git2::Error as Git2Error;

pub type CheckResult<T> = ::std::result::Result<T, CheckError>;
// custom build check error which contains MetadataError
#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    #[error("build check failed: {stderr}")]
    Check { stderr: String },
    #[error("error from `cargo_metadata`: {0}")]
    MetaData(#[from] MetaDataError),
    #[error("error from `git2`: {0}")]
    Git2(#[from] Git2Error),
}
