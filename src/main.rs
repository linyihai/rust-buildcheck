mod commands;
mod rules;

use clap::Parser;

fn main() {
    let args = commands::Args::parse();
    let checker = rules::BuildRuleChecker::new(args);
    if let Ok(checker) = checker {
        checker.check_rule();
    } else {
        println!("build check failed: {}", checker.err().unwrap());
    }
}
