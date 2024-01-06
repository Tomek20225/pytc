use std::fs;
mod utils;
use utils::types::TypeIdentifier;
use utils::reader::Reader;
use utils::processing::*;

fn main() {
    let filename = "./src/python/foo.pyc";
    let contents = fs::read(filename).expect("reading pyc file");
    let mut reader = Reader{
        current_idx: 0,
        contents
    };
    let code = process_code_block(&mut reader);
    println!("{:?}", code);

    // for byte in contents.iter() {
    //     let ch = *byte as char;
    //     println!("{byte} {ch}");
    // }
}