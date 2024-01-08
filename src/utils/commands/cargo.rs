use super::Args;
use crate::utils::custom_error::CheckResult;
use anyhow::Context;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn get_cargo_path() -> PathBuf {
    env::var("CARGO")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("cargo"))
}

pub fn pack_crate(args: &Args, manifest_path: &Path) -> CheckResult<()> {
    let mut command = Command::new(get_cargo_path());
    let _args = if args.offline {
        vec![
            "package",
            "--allow-dirty",
            "--no-verify",
            "--quiet",
            "--offline",
            "--manifest-path",
        ]
    } else {
        vec![
            "package",
            "--allow-dirty",
            "--no-verify",
            "--quiet",
            "--manifest-path",
        ]
    };

    command.args(_args).arg(manifest_path.as_os_str());
    command.stderr(Stdio::inherit());
    command.output().with_context(|| "cargo package failed")?;
    Ok(())
}
