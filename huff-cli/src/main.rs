use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use log::error;

use huff_cli::{run, Args};

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    let args = Args::parse();

    if let Err(e) = run(args) {
        error!("ERROR: {}", e);

        std::process::exit(1);
    }

    Ok(())
}
