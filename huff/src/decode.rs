use std::collections::HashMap;

use crate::binary_tree::Node;
use crate::huffman;

use anyhow::Result;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use bitvec::view::BitView;
use log::debug;

pub fn decode(input: &[u8]) -> Result<String> {
    let mut character_counts: HashMap<char, i32> = HashMap::new();
    let mut encoded_bits: BitVec<Msb0, u8> = BitVec::new();

    let mut read_chars = true;
    let mut reading_char = true;
    let mut last_char = String::new();
    let mut read_length = false;

    let mut encoding_lengths: [u8; 4] = [0; 4];
    let mut encoding_length_index: u32 = 0;

    for &byte in input {
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

    Ok(decoded_message)
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
                debug!("{:?} {:?}", *current_node, if bit { "1" } else { "0" });
            }

            if !bit {
                if let Some(left_node) = &current_node.left {
                    current_node = left_node;
                }
            } else if let Some(right_node) = &current_node.right {
                current_node = right_node;
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
    (array[0] as u32)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}
