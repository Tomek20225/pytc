use super::{code::CodeBlock, operations::Operation, var::Var};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::FunctionValue;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct LlvmCompiler {
    code: CodeBlock,
    refs: Vec<Var>,
}

// TODO: Make the IR generator use the instructions and refs it was given
impl LlvmCompiler {
    pub fn new(code: CodeBlock, refs: Vec<Var>) -> LlvmCompiler {
        LlvmCompiler { code, refs }
    }

    fn create_function<'a>(
        &'a self,
        context: &'a Context,
        module: &Module<'a>,
    ) -> FunctionValue<'_> {
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
        let module = context.create_module(&self.code.get_name(&self.refs));
        let builder = context.create_builder();

        let code_blocks = self.code.get_code_blocks(&self.refs);
        let mut stack: Vec<&Var> = vec![];
        let mut fn_idx = 0;

        // TODO: Objects are possibly CodeBlocks - in that case this code won't work as expected
        for code_block in code_blocks {
            // Function declaration
            let fn_ret_type = code_block.get_return_type(&self.refs, &context);
            let fn_type = fn_ret_type.fn_type(&[], false);
            let fn_name = if fn_idx > 0 {
                code_block.get_name(&self.refs)
            } else {
                String::from("main")
            };
            let function = module.add_function(&fn_name, fn_type, None);
            let entry = context.append_basic_block(function, "entry");
            builder.position_at_end(entry);

            // Variable preparation
            let consts = code_block.get_consts(&self.refs);
            let names = code_block.get_names(&self.refs);
            let mut variables: HashMap<&String, &Var> = HashMap::new();

            // Iteration through each operation of the code block
            for op in code_block.get_operations() {
                println!("{:?}: {:?}", code_block.get_name(&self.refs), op);

                match op {
                    Operation::LoadConst(i) => stack.push(consts[*i as usize]),
                    Operation::StoreName(i) => {
                        let name = &names[*i as usize];
                        let var = stack.pop().expect("stack to contain at least one element");
                        let var_type = var.get_type(&context);
                        variables.insert(name, var);

                        println!("declaring {:?}: {:?} = {:?}", name, var_type, var_type);

                        let llvm_var = builder
                            .build_alloca(var_type, name)
                            .expect("llvm to create a local pointer");
                        let llvm_var_val = match var_type {
                            BasicTypeEnum::IntType(t) => {
                                let value = var.as_int().expect("var of type int to be unpacked");
                                t.const_int(value as u64, false)
                            }
                            _ => todo!("declaring values of type {:?}", var_type),
                        };
                        builder.build_store(llvm_var, llvm_var_val).expect(&format!(
                            "llvm to declare a variable of type {:?}",
                            var_type
                        ));
                    }
                    Operation::LoadName(i) => {
                        let name = &names[*i as usize];
                        let var = *variables
                            .get(name)
                            .expect("loaded variable to be already declared");
                        stack.push(var);
                    }
                    _ => todo!("operation {:?}", op),
                }
            }

            fn_idx += 1;
        }

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
