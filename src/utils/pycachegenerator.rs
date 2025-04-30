use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct PyCacheGenerator;

impl PyCacheGenerator {
    pub fn new() -> Self {
        PyCacheGenerator
    }

    pub fn compile_py_to_pyc(&self, py_path: &Path) -> std::io::Result<PathBuf> {
        // Generate .pyc filename in the same directory
        let pyc_filename = format!("{}.pyc", py_path.file_stem().unwrap().to_str().unwrap());
        let pyc_path = py_path.parent().unwrap().join(pyc_filename);

        // Get Python version
        let python_version = Command::new("python3")
            .arg("-c")
            .arg("import sys; print(f'{sys.version_info.major}{sys.version_info.minor}')")
            .output()?;

        if !python_version.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get Python version"
            ));
        }

        let python_version = String::from_utf8_lossy(&python_version.stdout).trim().to_string();

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