use super::output::{self, Level};
use std::fmt::{self, Display};

#[derive(Debug, Default)]
pub struct Detail {
    pub description: String,
    pub level: output::Level,
    pub location: String,
    pub errno: usize,
    pub check_type: String,
    pub line: usize,
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
                // 用来识别具体规范编码，见[开发预定错误码](https://onebox.huawei.com/v/849036f19356874c5cf463c5c01dfe9d?type=1) 
                errno: 31004_usize,
                check_type: String::from("build tool"),
                ..Detail::default()
            },
            BuildRule::GRS06 => Detail {
                level: Level::Suggestion,
                errno: 31005_usize,
                check_type: String::from("build tool"),
                ..Detail::default()
            },
            BuildRule::GRS08 => Detail {
                level: Level::Rule,
                errno: 31007_usize,
                check_type: String::from("build configuration"),
                ..Detail::default()
            },
            BuildRule::GRS10 => Detail {
                level: Level::Rule,
                errno: 31009_usize,
                check_type: String::from("build configuration"),
                ..Detail::default()
            },
            BuildRule::GRS17 => Detail {
                level: Level::Rule,
                errno: 31016_usize,
                check_type: String::from("packaging and pushlishing"),
                ..Detail::default()
            },
            BuildRule::GRS18 => Detail {
                level: Level::Rule,
                errno: 31017_usize,
                check_type: String::from("packaging and pushlishing"),
                ..Detail::default()
            },
            BuildRule::GRS20 => Detail {
                level: Level::Rule,
                errno: 31019_usize,
                check_type: String::from("packaging and pushlishing"),
                ..Detail::default()
            },
        }
    }
}

impl Detail {
    pub fn build(
        build_rule: BuildRule,
        location: String,
        description: String,
        line: usize,
    ) -> Self {
        Self {
            location,
            description,
            line,
            ..Detail::from(build_rule)
        }
    }
}
