mod cargo;
pub use cargo::pack_crate;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// the path of Cargo.toml in the root package directory
    #[arg(short, long, default_value_t = String::from("Cargo.toml"))]
    pub manifest_path: String,
    /// the packed crate size(MB)
    #[arg(short, long, default_value_t = 10.0)]
    pub crate_size: f32,
}
