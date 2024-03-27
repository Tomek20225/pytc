use pytc::utils::llvm::LlvmCompiler;
use pytc::utils::reader::Reader;
use std::fs;
use std::process::Command;

#[test]
fn test_python_compilation() {
    let tests_root = "tests";

    for entry in fs::read_dir(tests_root).unwrap() {
        let test_case_dir = entry.unwrap().path();
        if test_case_dir.is_dir() {
            // Define paths within the test case directory
            let python_src_dir = test_case_dir.join("python_src");
            let output_dir = test_case_dir.join("output");

            // Ensure the output directory exists
            fs::create_dir_all(&output_dir).unwrap();

            // Step 0: Remove all files in the output directory
            for entry in fs::read_dir(&output_dir).unwrap() {
                let file = entry.unwrap().path();
                if file.is_file() {
                    fs::remove_file(file).unwrap();
                }
            }

            // Step 1: Generate .pyc files directly in the output directory
            for file in fs::read_dir(&python_src_dir).unwrap() {
                let py_file = file.unwrap().path();
                if py_file.extension().unwrap() == "py" {
                    // Convert the .py file to a .pyc file through the prepared script
                    let status = Command::new("python")
                        .args(&["-O", "./src/python/converter.py", py_file.to_str().unwrap()])
                        .status()
                        .expect("Failed to generate the .pyc file");

                    // If nothing crashed, it's a success
                    assert!(status.success());
                }
            }

            // Step 2: Move the .pyc files to the output directory
            for file in fs::read_dir(&python_src_dir).unwrap() {
                let pyc_file = file.unwrap().path();
                if pyc_file.extension().unwrap() == "pyc" {
                    fs::rename(&pyc_file, output_dir.join(pyc_file.file_name().unwrap())).unwrap();
                }
            }

            // Step 3: Process .pyc files with the compiler, generate the LLVM IR files (.ll)
            for file in fs::read_dir(&output_dir).unwrap() {
                let pyc_file = file.unwrap().path();
                if pyc_file.extension().unwrap() == "pyc" {
                    // Read the binary .pyc file
                    let contents =
                        fs::read(pyc_file.clone()).expect("Couldn't read the given file");

                    // Parse the binary .pyc file
                    let mut reader = Reader::new(contents);
                    let code = reader.read_file().expect("Couldn't parse the given file");
                    let refs = reader.get_refs().clone();

                    // Generate the LLVM IR
                    let llvm_compiler = LlvmCompiler::new(code, refs);
                    let llvm_ir = llvm_compiler.generate_ir();
                    llvm_compiler.save_to_file(&pyc_file, &llvm_ir);

                    // If nothing crashed, it's a success
                    assert!(true);
                }
            }

            // Step 4: Compile .ll files to executables, run them and check the results
            for file in fs::read_dir(&output_dir).unwrap() {
                let ll_file = file.unwrap().path();
                if ll_file.extension().unwrap() == "ll" {
                    let s_filename = &ll_file.to_str().unwrap().replace(".ll", ".s");
                    let exe_filename = &ll_file.to_str().unwrap().replace(".ll", "");

                    let status = Command::new("llc")
                        .args(&[ll_file.to_str().unwrap(), "-o", s_filename])
                        .status()
                        .expect("Failed to generate the .s file");
                    assert!(status.success());

                    // Compile executables with gcc
                    let status_gcc = Command::new("gcc")
                        .args(&[s_filename, "-o", exe_filename])
                        .status()
                        .expect("Failed to generate the .s file");
                    assert!(status_gcc.success());

                    // Run executables
                    let status_run = Command::new(exe_filename)
                        .status()
                        .expect("Failed to run the executable file");
                    assert!(status_run.success());

                    // TODO: Checl if the executable's result is correct
                }
            }
        }
    }
}
