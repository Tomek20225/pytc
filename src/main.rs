use std::fs;
use std::str;

// https://github.com/ThePrimeagen/ts-rust-zig-deez/blob/master/rust/src/lexer/lexer.rs#L175
// https://github.com/ThePrimeagen/ts-rust-zig-deez/blob/master/python/deez_py/tokens.py

enum TypeIdentifier {
    // Single Type
    None(u8),
    True(u8),

    // Short Type
    ShortAsciiInterned(u8),
    String(u8)
}

fn main() {
    let filename = "./src/python/foo.pyc";
    let contents = fs::read(filename).expect("reading pyc file");

    for byte in contents.iter() {
        let ch = *byte as char;
        println!("{byte} {ch}");
    }
}