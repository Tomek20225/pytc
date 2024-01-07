use std::fs;
use std::io::Write;
use std::path::Path;
mod utils;
use utils::reader::Reader;
use utils::transpiler::Transpiler;

fn main() {
    // Read the input file into the Vec of bytes
    let input_file: &str = "./src/python/foo.pyc"; // TODO: Make this assignable through CLI
    let file_path: &Path = Path::new(input_file);
    let contents = fs::read(file_path).expect("Couldn't read the given file");

    // Parse the binary .pyc file
    let mut reader = Reader {
        contents,
        ..Default::default()
    };
    let code = reader.read_file().expect("Couldn't parse the given file");
    println!("{:?}", code);

    // Transpile the code into C
    let transpiler = Transpiler {
        code
    };
    let c_str = transpiler.transpile_to_c();

    // Create a temporary C file to hold the transpiled code
    let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_dir = input_file.replace(file_name, "");
    let out_file = file_dir + file_stem + ".c";
    let out_file_path = Path::new(&out_file);
    let mut out = fs::File::create(out_file_path)
        .expect("Unable to create the temporary C file on disk in the given location");

    // Write to the C file
    out.write_all(c_str.as_bytes())
        .expect("Unable to write the data to the temporary C file");
    // println!("{:?}", out_file_path);
}
