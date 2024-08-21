use std::fs;
use std::path::Path;
use std::process::Command;

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

    // Generate assembly code from LLVM IR (llc)
    let ll_path = "./src/python/temp/foo.ll";
    let asm_path = "./src/python/temp/foo.s";

    let output = Command::new("llc")
        .arg(ll_path)
        .arg("-o")
        .arg(asm_path)
        .output()
        .expect("Failed to run llc");

    if !output.status.success() {
        panic!("llc failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Compile the assembly code into an executable (gcc)
    let exe_path = "./src/python/temp/foo";

    let output = Command::new("gcc")
        .arg(asm_path)
        .arg("-o")
        .arg(exe_path)
        .output()
        .expect("Failed to run gcc");

    if !output.status.success() {
        panic!("gcc failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Run the compiled executable
    let output = Command::new(exe_path)
        .output()
        .expect("Failed to run the executable");

    if !output.status.success() {
        panic!("Execution failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Print the output of the executable
    println!("\n==== OUTPUT FROM THE EXECUTABLE ====\n{}", String::from_utf8_lossy(&output.stdout));
}
