use super::operations::Operation;
use super::reader::CodeBlock;
use super::var::Var;

#[derive(Default, Debug)]
pub struct Transpiler {
    pub code: CodeBlock,
}

// c file structure
// ----------------
// {libs}
// {imports}
// {globals}
// {functions}
// {main}

impl Transpiler {
    fn get_libs(&self) -> String {
        let libs: Vec<&str> = vec!["stdio"];
        let mut libs_str = String::new();
        // TODO: Add logic
        for lib in libs.iter() {
            libs_str += &format!("#include <{}.h>\n", lib);
        }
        libs_str
    }

    fn get_imports(&self) -> String {
        let imports: Vec<&str> = vec![];
        let mut imports_str = String::new();
        // TODO: Add logic
        imports_str
    }

    fn get_globals(&self) -> String {
        let globals: Vec<&str> = vec![];
        let mut globals_str = String::new();
        // TODO: Add logic
        globals_str
    }

    fn get_functions(&self) -> String {
        let functions: Vec<&str> = vec![];
        let mut functions_str = String::new();
        // TODO: Add logic
        functions_str
    }

    fn get_main(&self) -> String {
        let main: Vec<&str> = vec![];
        let mut main_str = String::new();

        // TODO: Add logic
        main_str += "int main(){\n";
        main_str += "printf(\"Hello World\");\n";
        main_str += ";return 0;}";

        main_str
    }

    fn get_const(&self, idx: u8) -> Var {
        let co_const = &self.code.co_const;
        todo!();
        // co_const[idx]
    }

    fn parse_function(&self) -> String {
        let mut func_str = String::new();
        let mut const_stack: Vec<u8> = vec![]; // indexes in code.co_const
        let mut last_operation: &Operation = &Operation::default();

        for operation in self.code.co_code.iter() {
            match operation {
                Operation::LoadConst(idx) => const_stack.push(*idx),
                Operation::StoreName(name) => match last_operation {
                    Operation::LoadConst(idx) => {
                        let var = self.get_const(*idx);
                        todo!();
                    }
                    Operation::BinaryAdd(_) => {
                        todo!();
                    }
                    _ => continue,
                },
                _ => continue,
            }
            last_operation = operation;
        }

        func_str
    }

    fn parse_code_to_readable() {
        // TODO: Turn Var<SmallTuple<...>> into vec![...], with exception of Ref(3) (empty tuple)

        // TODO: Unpack the Var and Box<Var> wherever possible, or create helpers

        // TODO: Parse references, like:
        // - Ref(7) representing "baz" function object and name
        // - Ref(6) representing "foo.py" name(?)
        
        // TODO: Parse code objects to more understandable instrunctions?

        // TODO: Parse co_const to have name and value in one place? Create helper function?

        // TODO: Get inner functions into different places, like transpiler.functions Vec

        todo!()
    }

    pub fn transpile_to_c(&self) -> String {
        let mut c_str: String = String::new();

        c_str += &self.get_libs();
        c_str += &self.get_imports();
        c_str += &self.get_globals();
        c_str += &self.get_functions();
        c_str += &self.get_main();

        c_str.to_string()
    }
}
