use std::collections::HashMap;



use crate::huffman;
use anyhow::Result;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use log::debug;

fn char_occurences_in_string(string: &str) -> HashMap<char, i32> {
    let mut character_counts = HashMap::new();

    for char in string.chars() {
        *character_counts.entry(char).or_insert(0) += 1;
    }

    character_counts
}

pub fn encode(input: &str) -> Result<Vec<u8>> {
    let character_counts = char_occurences_in_string(&input);
    debug!("{:?}", character_counts);

    let huffman_graph = huffman::generate_tree(&character_counts);

    let character_codes = huffman::generate_codes(&huffman_graph);

    debug!("codes: {:?}", character_codes);

    let mut encoding = String::new();
    for char in input.chars() {
        encoding.push_str(character_codes.get(&char).unwrap());
    }

    // debug!("encoding: {}", encoding);

    // Write to out
    let mut io_buf = huffman::save_tree(character_counts);

    let mut encoding_buf: BitVec<Msb0, u8> = BitVec::new();
    let size: usize = encoding.len();
    debug!("encoding size: {}", size);
    encoding_buf.resize(size, false);

    for (index, c) in encoding.chars().enumerate() {
        if c == '1' {
            encoding_buf.set(index, true);
        } else if c == '0' {
            encoding_buf.set(index, false);
        }
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

    Ok(io_buf.to_vec())
}
