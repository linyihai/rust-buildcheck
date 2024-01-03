mod rules;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = utils::commands::Args::parse();
    let checker = rules::BuildRuleChecker::new(args)?;
    checker.check_rule()?;
    Ok(())
}
