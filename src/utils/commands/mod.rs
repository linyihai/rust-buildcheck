pub mod cargo;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The path of Cargo.toml
    #[arg(short, long, default_value_t = String::from("Cargo.toml"))]
    pub manifest_path: String,
    /// The output file
    #[arg(short, long, default_value_t = String::from("rust_buildcheck_output.json"))]
    pub output_file: String,
    /// Run without accessing the network
    #[arg(long)]
    pub offline: bool,
}
