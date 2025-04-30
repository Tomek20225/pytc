use super::{code::CodeBlock, operations::Operation, var::Var};
use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum, PointerValue};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct LlvmCompiler {
    code: CodeBlock,
    refs: Vec<Var>,
}

#[derive(Debug, Clone)]
pub struct LlvmVariable<'a> {
    v_type: BasicTypeEnum<'a>,
    ptr: BasicValueEnum<'a>,
    value: BasicValueEnum<'a>,
    var: Option<&'a Var>,
}

// TODO: Make the IR generator use the instructions and refs it was given
impl LlvmCompiler {
    pub fn new(code: CodeBlock, refs: Vec<Var>) -> LlvmCompiler {
        LlvmCompiler { code, refs }
    }

    pub fn generate_ir(&self) -> String {
        let context = Context::create();
        let module = context.create_module(&self.code.get_name(&self.refs));
        let builder = context.create_builder();

        let code_blocks = self.code.get_code_blocks(&self.refs);
        let mut stack: Vec<LlvmVariable> = vec![];
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
            let mut variables_ptr: HashMap<String, LlvmVariable> = HashMap::new();

            // Iteration through each operation of the code block
            for op in code_block.get_operations() {
                println!("{:?}: {:?}", code_block.get_name(&self.refs), op);

                match op {
                    Operation::LoadConst(i) => {
                        let name = String::from("temp");
                        let var = consts[*i as usize];

                        let var_type = match var {
                            Var::None => context.i32_type().as_basic_type_enum(), // defaults to 0
                            Var::Int(_) => context.i32_type().as_basic_type_enum(),
                            _ => todo!("can't get type of var {:?}", var),
                        };
                        let var_value = match var_type {
                            BasicTypeEnum::IntType(t) => {
                                let value = var.as_int().expect("var of type int to be unpacked");
                                let llvm_value = t.const_int(value as u64, false);
                                BasicValueEnum::IntValue(llvm_value)
                            }
                            _ => todo!("declaring values of type {:?}", var_type),
                        };

                        let llvm_ptr = builder
                            .build_alloca(var_type, &name)
                            .expect("llvm to create a local pointer");

                        let llvm_var = LlvmVariable {
                            ptr: BasicValueEnum::PointerValue(llvm_ptr),
                            v_type: var_type,
                            value: var_value,
                            var: Some(var),
                        };

                        variables_ptr.insert(name.clone(), llvm_var.clone());
                        stack.push(llvm_var);
                    }
                    Operation::StoreName(i) => {
                        let name = &names[*i as usize];
                        let llvm_var = stack.pop().expect("stack to contain at least one element");
                        llvm_var.ptr.set_name(name);
                        variables_ptr.remove("temp");
                        variables_ptr.insert(name.clone(), llvm_var.clone());

                        // println!("declaring {:?}: {:?} = {:?}", llvm_var, var_type, var_val);

                        builder
                            .build_store(llvm_var.ptr.into_pointer_value(), llvm_var.value)
                            .expect(&format!(
                                "llvm to declare a variable of type {:?}",
                                llvm_var.v_type
                            ));
                    }
                    Operation::LoadName(i) => {
                        let name = &names[*i as usize];
                        let llvm_var = variables_ptr
                            .get(name)
                            .expect("loaded variable to be already declared");

                        // TODO: This only supports i32s, while it should be able to handle all types
                        let llvm_val_ptr = builder
                            .build_load(
                                context.i32_type(),
                                llvm_var.ptr.into_pointer_value(),
                                &name,
                            )
                            .expect("llvm to load the variable");
                        let mut new_llvm_var = llvm_var.clone();
                        new_llvm_var.ptr = llvm_val_ptr;

                        stack.push(new_llvm_var);
                    }
                    Operation::BinaryAdd(_) => {
                        let b = stack
                            .pop()
                            .expect("stack to have the first of two elements");
                        let a = stack
                            .pop()
                            .expect("stack to have the second of two elements");

                        let a_val = a.ptr.into_int_value();
                        let b_val = b.ptr.into_int_value();

                        let llvm_val = builder
                            .build_int_add(a_val, b_val, "sum")
                            .expect("adding ints to work");
                        let llvm_ptr = builder
                            .build_alloca(context.i32_type(), "temp")
                            .expect("llvm to create a local pointer");

                        let result_var = LlvmVariable {
                            value: BasicValueEnum::IntValue(llvm_val),
                            v_type: context.i32_type().as_basic_type_enum(),
                            var: None,
                            ptr: llvm_ptr.as_basic_value_enum(),
                        };
                        stack.push(result_var);
                    }
                    Operation::ReturnValue(_) => {
                        let llvm_var = stack
                            .pop()
                            .expect("stack to contain at least one element");
                        let _ = builder.build_return(Some(&llvm_var.value));
                    }
                    _ => todo!("operation {:?}", op),
                }
            }

            fn_idx += 1;
        }

        module.print_to_string().to_string()
    }

    // TODO: Refactor this or export file reading/writing to a separate struct
    pub fn save_to_file(&self, file_path: &Path, ir: &str) {
        let input_file = file_path.to_str().unwrap();
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

    pub fn compile_to_assembly(&self, ll_path: &Path, asm_path: &Path) -> std::io::Result<()> {
        let output = Command::new("llc")
            .arg(ll_path)
            .arg("-o")
            .arg(asm_path)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("llc failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }

        Ok(())
    }

    pub fn compile_to_binary(&self, asm_path: &Path, bin_path: &Path) -> std::io::Result<()> {
        let output = Command::new("gcc")
            .arg(asm_path)
            .arg("-o")
            .arg(bin_path)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("gcc failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }

        Ok(())
    }

    pub fn execute_binary(&self, bin_path: &Path) -> std::io::Result<String> {
        let output = Command::new(bin_path)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Execution failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
