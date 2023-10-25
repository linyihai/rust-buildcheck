mod commands;
mod rules;

use clap::Parser;

fn main() {
    let args = commands::Args::parse();
    let checker = rules::BuildRuleChecker::new(args).unwrap();
    checker.check_rule();
}
