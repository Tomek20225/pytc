use std::path::Path;
use clap::Parser;

mod utils;
use utils::pycachereader::PyCacheReader;
use utils::llvm::LlvmCompiler;
use utils::pycachegenerator::PyCacheGenerator;

/// A tool to transpile Python bytecode to native code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input Python file
    #[arg(short, long)]
    input: String,
}

fn validate_input(input_path: &Path) -> Result<(), String> {
    if !input_path.is_file() {
        return Err("Error: Input must be a Python (.py) file".to_string());
    }

    if !input_path.extension().map_or(false, |ext| ext == "py") {
        return Err("Error: Input file must have .py extension".to_string());
    }

    Ok(())
}

fn compile_and_run(input_path: &Path) -> Result<String, String> {
    // Compile .py to .pyc
    let pyc_generator = PyCacheGenerator::new();
    let pyc_path = pyc_generator.compile_py_to_pyc(input_path)
        .map_err(|e| format!("Failed to compile Python file to bytecode: {}", e))?;
    
    // Read and process the .pyc file
    let mut reader = PyCacheReader::from_file(&pyc_path)
        .map_err(|e| format!("Couldn't read the .pyc file: {}", e))?;
    let code = reader.read_file()
        .ok_or_else(|| "Couldn't parse the .pyc file".to_string())?;
    let refs = reader.get_refs().clone();

    // Generate the LLVM IR
    let llvm_compiler = LlvmCompiler::new(code, refs);
    let llvm_ir = llvm_compiler.generate_ir();
    llvm_compiler.save_to_file(&pyc_path, &llvm_ir);

    // Generate assembly code from LLVM IR (llc)
    let ll_path = input_path.parent().unwrap().join(format!("{}.ll", input_path.file_stem().unwrap().to_str().unwrap()));
    let asm_path = input_path.parent().unwrap().join(format!("{}.s", input_path.file_stem().unwrap().to_str().unwrap()));

    llvm_compiler.compile_to_assembly(&ll_path, &asm_path)
        .map_err(|e| format!("Failed to compile LLVM IR to assembly: {}", e))?;

    // Compile the assembly code into an executable (gcc)
    let bin_path = input_path.parent().unwrap().join(input_path.file_stem().unwrap());

    llvm_compiler.compile_to_binary(&asm_path, &bin_path)
        .map_err(|e| format!("Failed to compile assembly to binary: {}", e))?;

    // Run the compiled executable
    llvm_compiler.execute_binary(&bin_path)
        .map_err(|e| format!("Failed to run the executable: {}", e))
}

fn main() {
    let args = Args::parse();
    let input_path = Path::new(&args.input);

    if let Err(e) = validate_input(input_path) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    match compile_and_run(input_path) {
        Ok(output) => println!("\n==== OUTPUT FROM THE EXECUTABLE ====\n{}", output),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
