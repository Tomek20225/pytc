use std::fs;
mod utils;
use utils::reader::Reader;

fn main() {
    let filename = "./src/python/foo.pyc";
    let contents = fs::read(filename).expect("Couldn't read the given file");

    let mut reader = Reader {
        contents,
        ..Default::default()
    };
    let code = reader.read_file().expect("Couldn't parse the given file");
    
    println!("{:?}", code);
}
