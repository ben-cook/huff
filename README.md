# Huff - A Huffman Coding CLI tool in Rust

Huff is a small Rust library and CLI tool that encodes and decodes text files using [Huffman Coding](https://en.wikipedia.org/wiki/Huffman_coding), like a much more simplified version of [gzip](https://en.wikipedia.org/wiki/Gzip). The file format is a custom format I came up with for this project.

The easiest way to run Huff is with [Cargo](https://github.com/rust-lang/cargo) installed. An example is provided that can be ran with `cargo run -- encode huff/tests/files/fox.txt`, and decoded with `cargo run -- decode out.huff`.
