use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::Edition;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::Spanned;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct CargoToml {
    pub package: PackageConfig,
    pub dependencies: Option<HashMap<String, Spanned<toml::Value>>>,
    pub dev_dependencies: Option<HashMap<String, Spanned<toml::Value>>>,
    pub build_dependencies: Option<HashMap<String, Spanned<toml::Value>>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PackageConfig {
    pub name: Option<Spanned<String>>,
    pub edition: Option<Spanned<Edition>>,
    pub license: Option<Spanned<String>>,
    pub license_file: Option<Spanned<Utf8PathBuf>>,
}

pub fn read_toml_file(path: &Path) -> (CargoToml, String) {
    let contents = fs::read_to_string(path).unwrap();
    (toml::from_str(&contents).unwrap(), contents)
}
