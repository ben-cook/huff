use std::{fs::read_to_string, path::Path};

use huff::encode;

fn evaluate_file(path: &Path) {
    let input = read_to_string(path).unwrap();
    let raw_len = input.len() as f32;
    let encoded_message = encode(&input).unwrap();
    let compressed_len = encoded_message.len() as f32;
    eprintln!(
        "{}: {} / {} = {}",
        path.display(),
        compressed_len,
        raw_len,
        compressed_len / raw_len
    );
}

#[test]
fn example_file() {
    evaluate_file(Path::new("tests/files/example.txt"));
}

#[test]
fn fox_file() {
    evaluate_file(Path::new("tests/files/fox.txt"));
}

#[test]
fn loremipsum_file() {
    evaluate_file(Path::new("tests/files/loremipsum.txt"));
}

#[test]
fn navy_file() {
    evaluate_file(Path::new("tests/files/navy.txt"));
}

#[test]
fn simple_file() {
    evaluate_file(Path::new("tests/files/simple.txt"));
}

#[test]
fn utf8_file() {
    evaluate_file(Path::new("tests/files/utf8.txt"));
}

#[test]
fn long_file() {
    evaluate_file(Path::new("tests/files/long.txt"));
}
