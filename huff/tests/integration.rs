use std::{fs::read_to_string, path::Path};

use huff::{decode::decode, encode::encode};

fn evaluate_file(path: &Path) {
    let input = read_to_string(path).unwrap();
    let encoded_message = &encode(&input).unwrap();
    let output = decode(encoded_message).unwrap();
    assert_eq!(input, output);
}

#[test]
fn example_file() {
    evaluate_file(Path::new("../examples/example.txt"));
}

#[test]
fn fox_file() {
    evaluate_file(Path::new("../examples/fox.txt"));
}

// #[test]
// fn loremipsum_file() {
//     evaluate_file(Path::new("../examples/loremipsum.txt"));
// }

#[test]
fn navy_file() {
    evaluate_file(Path::new("../examples/navy.txt"));
}

#[test]
fn simple_file() {
    evaluate_file(Path::new("../examples/simple.txt"));
}
