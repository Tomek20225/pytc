use std::fs;
use std::path::Path;
mod utils;
use utils::reader::Reader;
use utils::llvm::LlvmCompiler;
// use utils::transpiler::Transpiler;

fn main() {
    // Read the input file into the Vec of bytes
    // TODO: Make this assignable through CLI
    // TODO: Accept the .py file and use Python CLI to generate the .pyc
    // TODO: Accept the directory and find the .py files in there, create .pyc files and then transpile them
    let input_file: &str = "./src/python/temp/foo.pyc";
    let file_path: &Path = Path::new(input_file);
    println!("{:?}", file_path);
    let contents = fs::read(file_path).expect("Couldn't read the given file");

    // Parse the binary .pyc file
    let mut reader = Reader::new(contents);
    let code = reader.read_file().expect("Couldn't parse the given file");
    let refs = reader.get_refs().clone(); // TODO: Remove the unnecessary clone

    // Display debug info
    println!("REFS: {:?}\n", refs);
    println!("CODE: {:?}\n", code);

    // Generate the LLVM IR
    let llvm_compiler = LlvmCompiler::new(code, refs);
    let llvm_ir = llvm_compiler.generate_ir();
    llvm_compiler.save_to_file(file_path, &llvm_ir);

    // RUNNING THE CODE
    // llc ./src/python/foo.ll -o test.s
    // gcc test.s -o test
    // ./test
}
