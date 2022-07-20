use std::{fs::read_to_string, path::Path};

use huff::{decode, encode};

fn evaluate_file(path: &Path) {
    let input = read_to_string(path).unwrap();
    let encoded_message = encode(&input).unwrap();
    let output = decode(&encoded_message).unwrap();
    assert_eq!(input, output);
}

#[test]
fn example_file() {
    evaluate_file(Path::new("tests/integration_tests/example.txt"));
}

#[test]
fn fox_file() {
    evaluate_file(Path::new("tests/integration_tests/fox.txt"));
}

#[test]
fn loremipsum_file() {
    evaluate_file(Path::new("tests/integration_tests/loremipsum.txt"));
}

#[test]
fn navy_file() {
    evaluate_file(Path::new("tests/integration_tests/navy.txt"));
}

#[test]
fn simple_file() {
    evaluate_file(Path::new("tests/integration_tests/simple.txt"));
}

#[test]
fn utf8_file() {
    evaluate_file(Path::new("tests/integration_tests/utf8.txt"));
}
