use anyhow::Result;
use env_logger::Env;
use log::error;
use structopt::StructOpt;

use huff::{run, Cli};

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    let args = Cli::from_args();

    if let Err(e) = run(args) {
        error!("ERROR: {}", e);

        std::process::exit(1);
    }

    Ok(())
}
