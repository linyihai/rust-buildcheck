mod commands;
mod errors;
mod output;
mod rules;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = commands::Args::parse();
    let checker = rules::BuildRuleChecker::new(args)?;
    checker.check_rule();
    Ok(())
}
