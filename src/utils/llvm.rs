use super::{builtins, code::CodeBlock, operations::Operation, var::Var};
use crate::handle_print_builtin;
use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::BasicValueEnum;
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
    pub v_type: BasicTypeEnum<'a>,
    pub ptr: BasicValueEnum<'a>,
    pub value: BasicValueEnum<'a>,
}

// TODO: Make the IR generator use the instructions and refs it was given
impl LlvmCompiler {
    pub fn new(code: CodeBlock, refs: Vec<Var>) -> LlvmCompiler {
        LlvmCompiler { code, refs }
    }

    fn handle_load_const<'a>(
        &self,
        context: &'a Context,
        builder: &'a inkwell::builder::Builder,
        consts: &Vec<&Var>,
        i: u8,
        variables_ptr: &mut HashMap<String, LlvmVariable<'a>>,
        stack: &mut Vec<LlvmVariable<'a>>,
    ) {
        let name = String::from("temp");
        let var = consts[i as usize];

        let var_type = match var {
            Var::None => context.i32_type().as_basic_type_enum(), // defaults to 0
            Var::Int(_) => context.i32_type().as_basic_type_enum(),
            _ => todo!("can't get type of var {:?}", var),
        };
        let var_value = match var_type {
            BasicTypeEnum::IntType(t) => {
                let value = var.as_int().expect(&format!("expected var of type int to be unpacked - {:?}", var));
                let llvm_value = t.const_int(value as u64, false);
                BasicValueEnum::IntValue(llvm_value)
            }
            _ => todo!("declaring values of type {:?}", var_type),
        };

        let llvm_ptr = builder
            .build_alloca(var_type, &name)
            .expect(&format!("expected llvm to create a local pointer - {:?}", name));

        let llvm_var = LlvmVariable {
            ptr: BasicValueEnum::PointerValue(llvm_ptr),
            v_type: var_type,
            value: var_value,
        };

        variables_ptr.insert(name.clone(), llvm_var.clone());
        stack.push(llvm_var);
    }

    fn handle_store_name<'a>(
        &self,
        builder: &'a inkwell::builder::Builder,
        names: &[String],
        i: u8,
        variables_ptr: &mut HashMap<String, LlvmVariable<'a>>,
        stack: &mut Vec<LlvmVariable<'a>>,
    ) {
        let name = &names[i as usize];
        let llvm_var = stack.pop().expect(&format!("expected stack to contain at least one element - {:?}", name));
        llvm_var.ptr.set_name(name);
        variables_ptr.remove("temp");
        variables_ptr.insert(name.clone(), llvm_var.clone());

        builder
            .build_store(llvm_var.ptr.into_pointer_value(), llvm_var.value)
            .expect(&format!(
                "llvm to declare a variable of type {:?}",
                llvm_var.v_type
            ));
    }

    fn handle_load_name<'a>(
        &self,
        context: &'a Context,
        builder: &'a inkwell::builder::Builder,
        names: &[String],
        i: u8,
        variables_ptr: &HashMap<String, LlvmVariable<'a>>,
        stack: &mut Vec<LlvmVariable<'a>>,
    ) {
        let name = &names[i as usize];
        let llvm_var = match variables_ptr.get(name) {
            Some(var) => var,
            None => {
                // Check if it's a standard library function
                if builtins::is_builtin(name) {
                    // For builtin functions, we'll create a placeholder variable that represents the function
                    // This will be handled later in CallFunction
                    let builtin_var = builtins::create_builtin_placeholder(context);
                    stack.push(builtin_var);
                    return;
                } else {
                    // If it's not a builtin function, then it should be a user-defined variable or a function
                    panic!("expected loaded variable to be already declared - {:?}", name);
                }
            }
        };

        let llvm_val = builder
            .build_load(
                context.i32_type(),
                llvm_var.ptr.into_pointer_value(),
                &name,
            )
            .expect(&format!("expected llvm to load the variable - {:?}", name));
        
        let new_llvm_var = LlvmVariable {
            v_type: context.i32_type().as_basic_type_enum(),
            ptr: llvm_var.ptr,  // Keep the original pointer
            value: llvm_val,    // Store the loaded value
        };

        stack.push(new_llvm_var);
    }

    fn handle_binary_add<'a>(
        &self,
        context: &'a Context,
        builder: &'a inkwell::builder::Builder,
        stack: &mut Vec<LlvmVariable<'a>>,
    ) {
        let b = stack.pop().expect("expected stack to have the first of two elements");
        let a = stack.pop().expect("expected stack to have the second of two elements");

        let llvm_val = builder
            .build_int_add(a.value.into_int_value(), b.value.into_int_value(), "sum")
            .expect(&format!("expected adding ints to work - {:?}, {:?}", a.value, b.value));

        // Only create a new allocation if we need to store the result
        let result_var = LlvmVariable {
            value: BasicValueEnum::IntValue(llvm_val),
            v_type: context.i32_type().as_basic_type_enum(),
            ptr: a.ptr, // Reuse the pointer from the first operand
        };
        stack.push(result_var);
    }

    fn handle_binary_subtract<'a>(
        &self,
        context: &'a Context,
        builder: &'a inkwell::builder::Builder,
        stack: &mut Vec<LlvmVariable<'a>>,
    ) {
        let b = stack.pop().expect("expected stack to have the first of two elements");
        let a = stack.pop().expect("expected stack to have the second of two elements");

        let llvm_val = builder
            .build_int_sub(a.value.into_int_value(), b.value.into_int_value(), "sub")
            .expect("expected subtracting ints to work");

        // Only create a new allocation if we need to store the result
        let result_var = LlvmVariable {
            value: BasicValueEnum::IntValue(llvm_val),
            v_type: context.i32_type().as_basic_type_enum(),
            ptr: a.ptr, // Reuse the pointer from the first operand
        };
        stack.push(result_var);
    }

    fn handle_return_value<'a>(
        &self,
        builder: &'a inkwell::builder::Builder,
        stack: &mut Vec<LlvmVariable<'a>>,
    ) {
        let llvm_var = stack.pop().expect("expected stack to contain at least one element");
        let _ = builder.build_return(Some(&llvm_var.value));
    }

    fn handle_pop_top<'a>(
        &self,
        stack: &mut Vec<LlvmVariable<'a>>,
    ) {
        stack.pop().expect("expected stack to contain at least one element");
    }

    pub fn generate_ir(&self) -> String {
        let context = Context::create();
        let module = context.create_module(&self.code.get_name(&self.refs));
        let builder = context.create_builder();

        // Declare printf function at the beginning
        let printf_type = context.i32_type().fn_type(
            &[context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()],
            true, // varargs
        );
        let _printf_func = module.add_function("printf", printf_type, None);

        let code_blocks = self.code.get_code_blocks(&self.refs);
        let mut stack: Vec<LlvmVariable> = vec![];
        let mut fn_idx = 0;

        for code_block in code_blocks {
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

            let mut variables_ptr: HashMap<String, LlvmVariable> = HashMap::new();
            let names = code_block.get_names(&self.refs);
            let consts = code_block.get_consts(&self.refs);
            let operations = code_block.get_operations();

            for op in operations {
                match op {
                    Operation::LoadConstArg(i) => {
                        self.handle_load_const(&context, &builder, &consts, *i, &mut variables_ptr, &mut stack);
                    }
                    Operation::StoreNameArg(i) => {
                        self.handle_store_name(&builder, &names, *i, &mut variables_ptr, &mut stack);
                    }
                    Operation::LoadNameArg(i) => {
                        self.handle_load_name(&context, &builder, &names, *i, &variables_ptr, &mut stack);
                    }
                    Operation::BinaryAdd => {
                        self.handle_binary_add(&context, &builder, &mut stack);
                    }
                    Operation::BinarySubtract => {
                        self.handle_binary_subtract(&context, &builder, &mut stack);
                    }
                    Operation::ReturnValue => {
                        self.handle_return_value(&builder, &mut stack);
                    }
                    Operation::StopCode => {
                        // StopCode marks the end of bytecode - ignore it
                        continue;
                    }
                    Operation::PopTop => {
                        self.handle_pop_top(&mut stack);
                    }
                    Operation::CallFunctionArg(i) => {
                        // Handle function calls using the builtins module
                        let arg_count = *i;
                        
                        if stack.len() < (arg_count + 1) as usize {
                            panic!("expected stack to have at least {} arguments plus function name", arg_count);
                        }
                        
                        // Get the argument (it's the top of the stack)
                        let arg = stack.pop().expect("expected argument on stack");
                        
                        // Get the function name (it's now the top of the stack)
                        let func_name_var = stack.pop().expect("expected function name on stack");
                        
                        // Find the function name in the names list
                        let func_name = builtins::find_function_name(&names, &variables_ptr, &func_name_var)
                            .expect("expected to find function name in variables or built-ins");
                        
                        // Check if it's a builtin function and handle it
                        if builtins::is_builtin(&func_name) {
                            // Handle builtin function call using the builtins module
                            match func_name.as_str() {
                                builtins::PRINT => {
                                    handle_print_builtin!(builder, module, arg);
                                }
                                _ => {
                                    todo!("Builtin function '{}' not yet implemented", func_name);
                                }
                            }
                            
                            // Push a dummy result back onto the stack to maintain stack balance
                            let dummy_result = LlvmVariable {
                                v_type: context.i32_type().as_basic_type_enum(),
                                ptr: context.i32_type().const_zero().into(),
                                value: context.i32_type().const_zero().into(),
                            };
                            stack.push(dummy_result);
                        } else {
                            // For non-builtin functions, create a dummy result for now
                            let dummy_result = LlvmVariable {
                                v_type: context.i32_type().as_basic_type_enum(),
                                ptr: context.i32_type().const_zero().into(),
                                value: context.i32_type().const_zero().into(),
                            };
                            stack.push(dummy_result);
                            todo!("Function call to {} (not yet fully implemented)", func_name);
                        }
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
            .expect(&format!("Unable to create the LLVM IR file on disk in the given location - {:?}", out_file_path.display()));

        out.write_all(ir.as_bytes())
            .expect(&format!("Unable to write the data to the LLVM IR file - {:?}", out_file_path.display()));
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
