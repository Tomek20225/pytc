use super::reader::CodeBlock;

#[derive(Default, Debug)]
pub struct Transpiler {
    pub code: CodeBlock
}

impl Transpiler {
    pub fn transpile_to_c(&self) -> String {
        let c_str: &str = "#include <stdio.h>\nint main(){printf(\"Hello World\");\n;return 0;}";
        c_str.to_string()
    }
}