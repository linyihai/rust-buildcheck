use crate::errors::CheckError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Output {
    build_check_type: String,
    version: String,
    name: String,
    errlist: Vec<ErrDescription>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ErrDescription {
    defect_type: String,
    detail: String,
    errno: String,
    level: Level,
    line: u32,
    location: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub enum Level {
    #[default]
    #[serde(rename = "suggestion")]
    Suggestion,
    #[serde(rename = "rule")]
    Rule,
}

impl From<Vec<&CheckError>> for Output {
    fn from(value: Vec<&CheckError>) -> Self {
        Self {
            errlist: value
                .iter()
                .filter_map(|v| match v {
                    CheckError::CheckDetail(detail) => Some(ErrDescription {
                        detail: detail.description.clone(),
                        location: detail.location.clone(),
                        level: detail.level,
                        errno: detail.errno.clone(),
                        ..Default::default()
                    }),
                    _ => None,
                })
                .collect(),
            ..Self::default()
        }
    }
}

impl Default for Output {
    fn default() -> Self {
        Self {
            build_check_type: "check_build_standard".to_string(),
            version: "0.1.0".to_string(),
            name: "rust-buildcheck".to_string(),
            errlist: Vec::new(),
        }
    }
}
