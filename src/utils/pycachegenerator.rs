use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct PyCacheGenerator;

impl PyCacheGenerator {
    pub fn new() -> Self {
        PyCacheGenerator
    }

    fn generate_instructions_file(&self, py_path: &Path) -> std::io::Result<PathBuf> {
        // Generate .instructions filename in the same directory
        let instructions_filename = format!("{}.instructions", py_path.file_stem().unwrap().to_str().unwrap());
        let instructions_path = py_path.parent().unwrap().join(instructions_filename);

        // Run Python disassembly script
        let python_script = format!(
            r#"import marshal, types, dis, sys
with open('{}', 'r') as f:
    code = compile(f.read(), '{}', 'exec')
with open('{}', 'w') as f:
    f.write('=== Main Code Object ===\n')
    dis.dis(code, file=f)
    f.write('\n\n=== Code Object Fields ===\n')
    for name in dir(code):
        if name.startswith('co_'):
            f.write(f'{{name:<20}} = {{getattr(code, name)}}\n')
    f.write('\n\n=== Sub Code Objects ===\n')
    for const in code.co_consts:
        if isinstance(const, types.CodeType):
            f.write(f'\nDisassembly of {{const.co_name}}:\n')
            dis.dis(const, file=f)"#,
            py_path.to_str().unwrap(),
            py_path.to_str().unwrap(),
            instructions_path.to_str().unwrap()
        );

        let output = Command::new("python3")
            .arg("-c")
            .arg(python_script)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to generate instructions file: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }

        Ok(instructions_path)
    }

    pub fn compile_py_to_pyc(&self, py_path: &Path) -> std::io::Result<PathBuf> {
        // Generate .pyc filename in the same directory
        let pyc_filename = format!("{}.pyc", py_path.file_stem().unwrap().to_str().unwrap());
        let pyc_path = py_path.parent().unwrap().join(pyc_filename);

        // Get Python version
        let python_version = Command::new("python3")
            .arg("-c")
            .arg(r#"import sys; print(f"{sys.version_info.major}{sys.version_info.minor}")"#)
            .output()?;

        if !python_version.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get Python version"
            ));
        }

        let python_version = String::from_utf8_lossy(&python_version.stdout).trim().to_string();

        // Generate instructions file
        self.generate_instructions_file(py_path)?;

        // Compile Python file to bytecode
        let output = Command::new("python3")
            .arg("-m")
            .arg("py_compile")
            .arg(py_path)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Python compilation failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }

        // The py_compile module creates the .pyc file in a __pycache__ directory
        let cache_dir = py_path.parent().unwrap().join("__pycache__");
        let cache_file = cache_dir.join(format!(
            "{}.cpython-{}.pyc",
            py_path.file_stem().unwrap().to_str().unwrap(),
            python_version
        ));

        if cache_file.exists() {
            fs::rename(&cache_file, &pyc_path)?;
            // Only try to remove the directory if it exists and is empty
            if cache_dir.exists() && cache_dir.read_dir()?.next().is_none() {
                fs::remove_dir(cache_dir)?;
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Could not find compiled .pyc file at {:?}", cache_file)
            ));
        }

        Ok(pyc_path)
    }
} 