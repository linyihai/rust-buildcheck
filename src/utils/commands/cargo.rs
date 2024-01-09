use crate::utils::custom_error::CheckResult;
use anyhow::Context;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_cargo_command() -> Command {
    let cargo_path = env::var("CARGO")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("cargo"));
    Command::new(cargo_path)
}

pub fn cargo_package(manifest_path: &Path) -> CheckResult<()> {
    let mut command = get_cargo_command();
    command
        .args([
            "package",
            "--allow-dirty",
            "--no-verify",
            "--quiet",
            "--offline",
            "--manifest-path",
        ])
        .arg(manifest_path.as_os_str());
    command.output().with_context(|| "cargo package failed")?;
    Ok(())
}

pub fn cargo_fetch(manifest_path: &Path) -> CheckResult<()> {
    let mut command = get_cargo_command();
    command
        .args(["fetch", "--quiet", "--manifest-path"])
        .arg(manifest_path);
    command.output().with_context(|| "cargo fetch failed")?;
    Ok(())
}
