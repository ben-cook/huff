use std::{collections::HashMap, fs::File, io::Write, str};

use anyhow::{Context, Result};
use bitvec::prelude::*;
use log::{debug, info};
use structopt::StructOpt;

mod binary_tree;
use binary_tree::Node;

mod huffman;

#[derive(PartialEq)]
pub enum Mode {
    Encode,
    Decode,
}

impl str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "encode" => return Ok(Mode::Encode),
            "decode" => return Ok(Mode::Decode),
            _ => {
                return Err(format!("Could not parse mode '{}'", s));
            }
        };
    }
}

#[derive(StructOpt)]
pub struct Cli {
    pub mode: Mode,
    #[structopt(parse(from_os_str))]
    pub input_path: std::path::PathBuf,
}

pub fn run(args: Cli) -> Result<()> {
    if args.mode == Mode::Encode {
        return encode(args);
    } else if args.mode == Mode::Decode {
        return decode(args);
    }
    Ok(())
}

fn encode(args: Cli) -> Result<()> {
    info!("Reading file {:?}", &args.input_path);
    let content = std::fs::read_to_string(&args.input_path)
        .with_context(|| format!("Could not read file {:?}", &args.input_path))?;

    let characters: Vec<char> = content.chars().collect();

    let character_counts = get_character_counts(&characters);
    debug!("{:?}", character_counts);

    let huffman_graph = huffman::generate_tree(&character_counts);

    let character_codes = huffman::generate_codes(&huffman_graph);

    debug!("codes: {:?}", character_codes);

    let mut encoding = String::new();
    for char in characters {
        encoding.push_str(character_codes.get(&char).unwrap());
    }

    // debug!("encoding: {}", encoding);

    // Write to out
    let mut file = File::create("out.huff")?;
    let mut io_buf = huffman::save_tree(character_counts);

    let mut encoding_buf: BitVec<Msb0, u8> = BitVec::new();
    let size: usize = encoding.len();
    debug!("encoding size: {}", size);
    encoding_buf.resize(size, false);

    let mut index: usize = 0;

    for c in encoding.chars() {
        if c == '1' {
            encoding_buf.set(index, true);
        } else if c == '0' {
            encoding_buf.set(index, false);
        }
        index += 1;
    }

    // debug!("encoding_buf: {:?}", encoding_buf);

    // add a buffer of 00000000
    io_buf.extend(vec![0u8]);
    // add a buffer of length of encoding
    let encoding_len = encoding_buf.len() as u32;
    let encoding_len_buf: Vec<u8> = Vec::from(encoding_len.to_le_bytes());
    io_buf.extend(encoding_len_buf);
    // add the actual encoding
    io_buf.extend(encoding_buf.into_vec());

    file.write_all(&io_buf).expect("couldnt write");

    Ok(())
}

fn decode(args: Cli) -> Result<()> {
    let file_data = std::fs::read(&args.input_path)
        .with_context(|| format!("could not read file {:?}", &args.input_path))?;

    let mut character_counts: HashMap<char, i32> = HashMap::new();
    let mut encoded_bits: BitVec<Msb0, u8> = BitVec::new();

    let mut read_chars = true;
    let mut reading_char = true;
    let mut last_char = String::new();
    let mut read_length = false;

    let mut encoding_lengths: [u8; 4] = [0; 4];
    let mut encoding_length_index: u32 = 0;

    for byte in file_data.into_iter() {
        if read_length {
            encoding_lengths[encoding_length_index as usize] = byte;
            encoding_length_index += 1;
            if encoding_length_index == 4 {
                read_length = false;
                debug!("encoding lengths: {:?}", encoding_lengths);
            }
            continue;
        }

        if read_chars {
            if reading_char {
                if byte == 0u8 {
                    read_chars = false;
                    read_length = true;
                    continue;
                } else {
                    let byte_vec: Vec<u8> = Vec::from([byte]);
                    last_char = String::from_utf8(byte_vec).expect("couldn't parse character");
                    reading_char = false;
                }
            } else {
                let char_count = byte as i32;
                character_counts.insert(
                    last_char
                        .chars()
                        .into_iter()
                        .next()
                        .expect("Couldn't get char from string"),
                    char_count,
                );
                reading_char = true;
            }
        } else {
            let mut bits: BitVec<Msb0, u8> = byte.view_bits::<Msb0>().to_bitvec();

            encoded_bits.append(&mut bits);
        }
    }

    let encoding_length = u8_to_u32(&encoding_lengths);
    debug!("encoding length: {}", encoding_length);
    debug!("{:?}", character_counts);
    // debug!("{:?}", encoded_bits);

    let huffman_graph = huffman::generate_tree(&character_counts);

    // The decoding algorithm is somtimes generating different character codes than the encoding algorithm,
    // leading to incorrect decoding.
    let character_codes = huffman::generate_codes(&huffman_graph);
    debug!("{:?}", character_codes);

    let decoded_message = decode_message(encoded_bits, huffman_graph, encoding_length);

    println!("{}", decoded_message);
    Ok(())
}

fn get_character_counts(string: &Vec<char>) -> HashMap<char, i32> {
    let mut character_counts: HashMap<char, i32> = HashMap::new();

    for char in string {
        *character_counts.entry(*char).or_insert(0) += 1;
    }

    character_counts
}

fn decode_message(
    encoded_msg: BitVec<Msb0, u8>,
    root: Node<(i32, Option<char>)>,
    encoding_length: u32,
) -> String {
    let mut decoded_message = String::new();
    let mut current_node = &root;
    let mut current_length = 0;

    for bit in encoded_msg.into_iter() {
        if current_length < encoding_length {
            if current_length < 20 {
                debug!("{} {}", *current_node, if bit { "1" } else { "0" });
            }

            if bit == false {
                if let Some(left_node) = &current_node.left {
                    current_node = &left_node;
                }
            } else {
                if let Some(right_node) = &current_node.right {
                    current_node = &right_node;
                }
            }

            if let Some(char) = current_node.value.1 {
                decoded_message.push(char);
                current_node = &root;
                if current_length < 20 {
                    debug!("found char {}", char);
                }
            }
            current_length += 1;
        }
    }

    decoded_message
}

fn u8_to_u32(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}
