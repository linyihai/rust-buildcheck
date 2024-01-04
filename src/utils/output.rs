use super::custom_error::CheckError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Output {
    pub build_check_type: String,
    pub version: String,
    pub name: String,
    pub errlist: Vec<ErrDescription>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ErrDescription {
    pub defect_type: String,
    pub detail: String,
    pub errno: usize,
    pub level: Level,
    pub line: usize,
    pub location: String,
    #[serde(rename = "type")]
    pub check_type: String,
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
                        check_type: detail.check_type.clone(),
                        errno: detail.errno,
                        line: detail.line,
                        level: detail.level,
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
