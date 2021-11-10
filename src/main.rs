use structopt::StructOpt;
use anyhow::Result;

use huff::{Cli, run};

fn main() -> Result<()> {
    let args = Cli::from_args();

    if let Err(e) = run(args) {
        println!("Application error: {}", e);

        std::process::exit(1);
    }
    
    Ok(())
}
