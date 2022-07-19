use std::collections::HashMap;
use std::io::{self, Read};

use crate::binary_tree::Node;
use crate::huffman;

use anyhow::Result;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use log::debug;

/// Decodes (ascii) character counts into a Map<character, count>.
/// The input is a u8 slice that alternates between the ascii character code
/// and the count of that character
fn decode_character_counts(input: &[u8]) -> Result<HashMap<char, i32>> {
    let mut character_counts: HashMap<char, i32> = HashMap::new();

    let mut cursor = io::Cursor::new(input);

    loop {
        let mut char_buf = [0; 1];
        if cursor.read_exact(&mut char_buf).is_err() {
            break;
        }

        let character = char_buf[0].into();

        let count = match leb128::read::unsigned(&mut cursor) {
            Ok(value) => value.try_into()?,
            Err(_) => break,
        };

        character_counts.insert(character, count);
    }

    Ok(character_counts)
}

pub fn decode(input: &[u8]) -> Result<String> {
    let split_index = input.iter().position(|&byte| byte == 0x00).unwrap();
    let (character_slice, message_slice) = input.split_at(split_index);
    let message_slice = &message_slice[1..];

    let character_counts = decode_character_counts(&character_slice)?;
    dbg!(&character_counts);
    debug!("{:?}", character_counts);

    let (length, message) = message_slice.split_at(std::mem::size_of::<u32>());
    let message_len = u32::from_be_bytes(length.try_into().unwrap());
    let message: BitVec<Msb0, u8> = BitVec::from_slice(message)?;

    // debug!("{:?}", encoded_bits);

    let huffman_graph = huffman::generate_tree(&character_counts);

    // The decoding algorithm is somtimes generating different character codes than the encoding algorithm,
    // leading to incorrect decoding.
    let character_codes = huffman::generate_codes(&huffman_graph);
    dbg!(character_codes);

    let decoded_message = decode_message(message, huffman_graph, message_len);

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
                // debug!("{:?} {:?}", *current_node, if bit { "1" } else { "0" });
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode;
    use std::fs::read_to_string;

    #[test]
    fn decode_character_counts_tests() {
        let code = [97, 1, 98, 2, 99, 3, 100];
        let mut map = HashMap::new();
        map.insert('a', 1);
        map.insert('b', 2);
        map.insert('c', 3);

        assert_eq!(decode_character_counts(&code).unwrap(), map);
        assert_eq!(
            decode_character_counts(&code[..code.len() - 1]).unwrap(),
            map
        );
    }

    #[test]
    fn sanity_check() {
        let input = read_to_string("tests/integration_tests/loremipsum.txt").unwrap();
        let encode_chars = encode::char_occurences_in_string(&input);
        let encoded_message = encode(&input).unwrap();

        let split_index = encoded_message
            .iter()
            .position(|&byte| byte == 0x00)
            .unwrap();
        let (character_slice, _) = encoded_message.split_at(split_index);

        dbg!(&character_slice);
        let decode_chars = decode_character_counts(&character_slice).unwrap();

        for key in encode_chars.keys() {
            if encode_chars.get(&key) != decode_chars.get(&key) {
                println!("discrepency: {:?}", key);
            }
        }
        assert_eq!(encode_chars, decode_chars);
    }
}
