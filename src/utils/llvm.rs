use super::{operations::Operation, reader::CodeBlock, var::Var};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::FunctionValue;
use std::path::Path;
use std::fs;
use std::io::Write;

#[derive(Debug)]
pub struct LlvmCompiler {
    instructions: CodeBlock,
    refs: Vec<Var>,
}

// TODO: Make the IR generator use the instructions and refs it was given
impl LlvmCompiler {
    pub fn new(instructions: CodeBlock, refs: Vec<Var>) -> LlvmCompiler {
        LlvmCompiler {
            instructions,
            refs,
        }
    }

    fn create_function<'a>(&'a self, context: &'a Context, module: &Module<'a>) -> FunctionValue<'_> {
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("main", fn_type, None);
    
        let basic_block = context.append_basic_block(function, "entry");
        function
    }

    fn add_instructions(&self, builder: &Builder<'_>, function: FunctionValue<'_>) {
        let entry = function.get_first_basic_block().unwrap();
        builder.position_at_end(entry);
    
        let i32_type = function.get_type().get_context().i32_type();
        let const_int = i32_type.const_int(42, false);
        builder.build_return(Some(&const_int));
    }

    pub fn generate_ir(&self) -> String {
        let context = Context::create();
        let module = context.create_module("my_module");
        let builder = context.create_builder();

        let function = self.create_function(&context, &module);
        self.add_instructions(&builder, function);

        module.print_to_string().to_string()
    }

    // TODO: Refactor this or export file reading/writing to a separate struct
    pub fn save_to_file(&self, input_file: &str, file_path: &Path, ir: &str) {
        let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let file_dir = input_file.replace(file_name, "");
        let out_file = file_dir + file_stem + ".ll";
        let out_file_path = Path::new(&out_file);
        let mut out = fs::File::create(out_file_path)
            .expect("Unable to create the LLVM IR file on disk in the given location");

        out.write_all(ir.as_bytes())
            .expect("Unable to write the data to the LLVM IR file");
        println!("SAVED THE LLVM IR TO: {:?}", out_file_path);
    }
}