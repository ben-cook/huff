use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::{ArgEnum, Parser};

use huff::decode::decode;
use huff::encode::encode;

#[derive(PartialEq, Clone, ArgEnum)]
pub enum Mode {
    Encode,
    Decode,
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(arg_enum)]
    pub mode: Mode,
    pub input_path: PathBuf,
}

pub fn run(args: Args) -> Result<()> {
    match args.mode {
        Mode::Encode => {
            let input = read_to_string(args.input_path)?;
            let encoded_message = encode(&input)?;
            let mut file = File::create("out.huff")?;
            file.write_all(&encoded_message)
                .map_err(anyhow::Error::from)
        }
        Mode::Decode => {
            let input = read_to_string(args.input_path)?;
            let decoded_message = decode(&input.as_bytes())?;
            println!("{}", decoded_message);
            Ok(())
        }
    }
}
