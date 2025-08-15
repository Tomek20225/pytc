use super::{builtins, code::CodeBlock, operations::Operation, var::Var};
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

// Owned type that doesn't depend on lifetimes
#[derive(Debug, Clone)]
pub enum VarType {
    Int32,
    // Add more types as needed
}

// Owned variable representation without lifetime dependencies
#[derive(Debug, Clone)]
pub struct OwnedVariable {
    pub var_type: VarType,
    pub name: String,
    pub value: i64, // Store the actual value, not LLVM representation
    pub is_temp: bool,
}

// Runtime LLVM variable that gets created during IR generation
#[derive(Debug, Clone)]
pub struct LlvmVariable<'a> {
    pub v_type: BasicTypeEnum<'a>,
    pub ptr: BasicValueEnum<'a>,
    pub value: BasicValueEnum<'a>,
}

// Handler struct that can be extracted without lifetime issues
pub struct LlvmHandlers {
    temp_counter: usize,
}

impl LlvmHandlers {
    pub fn new() -> Self {
        Self { temp_counter: 0 }
    }

    fn get_next_temp_name(&mut self) -> String {
        let name = format!("temp_{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    pub fn handle_load_const(
        &mut self,
        consts: &[&Var],
        i: u8,
        variables: &mut HashMap<String, OwnedVariable>,
        stack: &mut Vec<OwnedVariable>,
    ) {
        let temp_name = self.get_next_temp_name();
        let var = consts[i as usize];
        let owned_var = OwnedVariable::from_var(var, temp_name.clone(), true);
        
        variables.insert(temp_name, owned_var.clone());
        stack.push(owned_var);
    }

    pub fn handle_store_name(
        &mut self,
        names: &[String],
        i: u8,
        variables: &mut HashMap<String, OwnedVariable>,
        stack: &mut Vec<OwnedVariable>,
    ) {
        let name = &names[i as usize];
        let mut temp_var = stack.pop().expect(&format!("expected stack to contain at least one element - {:?}", name));
        
        // Update the variable to be non-temporary and use the proper name
        temp_var.name = name.clone();
        temp_var.is_temp = false;
        
        // Remove any temporary variables
        let temp_keys: Vec<String> = variables.keys()
            .filter(|k| k.starts_with("temp_"))
            .cloned()
            .collect();
        for temp_key in temp_keys {
            variables.remove(&temp_key);
        }
        
        variables.insert(name.clone(), temp_var);
    }

    pub fn handle_load_name(
        &self,
        names: &[String],
        i: u8,
        variables: &HashMap<String, OwnedVariable>,
        stack: &mut Vec<OwnedVariable>,
    ) {
        let name = &names[i as usize];
        
        if let Some(var) = variables.get(name) {
            stack.push(var.clone());
        } else if builtins::is_builtin(name) {
            // Create a placeholder for builtin functions
            let builtin_var = OwnedVariable {
                var_type: VarType::Int32,
                name: name.clone(),
                value: 0,
                is_temp: false,
            };
            stack.push(builtin_var);
        } else {
            panic!("expected loaded variable to be already declared - {:?}", name);
        }
    }

    pub fn handle_binary_add(
        &mut self,
        stack: &mut Vec<OwnedVariable>,
    ) {
        let b = stack.pop().expect("expected stack to have the first of two elements");
        let a = stack.pop().expect("expected stack to have the second of two elements");

        let result_value = a.value + b.value;
        let temp_name = self.get_next_temp_name();
        
        let result_var = OwnedVariable {
            var_type: VarType::Int32,
            name: temp_name,
            value: result_value,
            is_temp: true,
        };
        
        stack.push(result_var);
    }

    pub fn handle_binary_subtract(
        &mut self,
        stack: &mut Vec<OwnedVariable>,
    ) {
        let b = stack.pop().expect("expected stack to have the first of two elements");
        let a = stack.pop().expect("expected stack to have the second of two elements");

        let result_value = a.value - b.value;
        let temp_name = self.get_next_temp_name();
        
        let result_var = OwnedVariable {
            var_type: VarType::Int32,
            name: temp_name,
            value: result_value,
            is_temp: true,
        };
        
        stack.push(result_var);
    }

    pub fn handle_return_value(
        &self,
        stack: &mut Vec<OwnedVariable>,
    ) -> Option<OwnedVariable> {
        stack.pop()
    }

    pub fn handle_pop_top(
        &self,
        stack: &mut Vec<OwnedVariable>,
    ) {
        stack.pop().expect("expected stack to contain at least one element");
    }

    pub fn handle_call_function(
        &mut self,
        names: &[String],
        arg_count: u8,
        variables: &HashMap<String, OwnedVariable>,
        stack: &mut Vec<OwnedVariable>,
    ) -> Result<(String, Vec<OwnedVariable>), String> {
        if stack.len() < (arg_count + 1) as usize {
            return Err(format!("expected stack to have at least {} arguments plus function name", arg_count));
        }
        
        // Get the arguments (they're on top of the stack)
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(stack.pop().expect("expected argument on stack"));
        }
        args.reverse(); // Arguments were pushed in reverse order
        
        // Get the function name (it's now the top of the stack)
        let func_name_var = stack.pop().expect("expected function name on stack");
        
        // Find the function name
        let func_name = names.iter()
            .find(|name| {
                if let Some(var) = variables.get(*name) {
                    var.name == func_name_var.name
                } else {
                    builtins::is_builtin(name)
                }
            })
            .cloned()
            .unwrap_or_else(|| func_name_var.name.clone());
        
        // Push a dummy result back onto the stack to maintain stack balance
        let temp_name = self.get_next_temp_name();
        let dummy_result = OwnedVariable {
            var_type: VarType::Int32,
            name: temp_name,
            value: 0,
            is_temp: true,
        };
        stack.push(dummy_result);
        
        Ok((func_name, args))
    }
}

impl VarType {
    fn to_llvm_type<'a>(&self, context: &'a Context) -> BasicTypeEnum<'a> {
        match self {
            VarType::Int32 => context.i32_type().as_basic_type_enum(),
        }
    }
}

impl OwnedVariable {
    fn to_llvm_value<'a>(&self, context: &'a Context) -> BasicValueEnum<'a> {
        match self.var_type {
            VarType::Int32 => {
                let int_type = context.i32_type();
                BasicValueEnum::IntValue(int_type.const_int(self.value as u64, false))
            }
        }
    }

    fn from_var(var: &Var, name: String, is_temp: bool) -> Self {
        match var {
            Var::None => OwnedVariable {
                var_type: VarType::Int32,
                name,
                value: 0,
                is_temp,
            },
            Var::Int(val) => OwnedVariable {
                var_type: VarType::Int32,
                name,
                value: *val as i64,
                is_temp,
            },
            _ => todo!("Support for var type {:?} not implemented", var),
        }
    }
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

        // Declare printf function at the beginning
        let printf_type = context.i32_type().fn_type(
            &[context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()],
            true, // varargs
        );
        let _printf_func = module.add_function("printf", printf_type, None);

        let code_blocks = self.code.get_code_blocks(&self.refs);
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

            // Use owned variables and handlers
            let mut handlers = LlvmHandlers::new();
            let mut variables: HashMap<String, OwnedVariable> = HashMap::new();
            let mut stack: Vec<OwnedVariable> = Vec::new();
            let mut llvm_variables: HashMap<String, LlvmVariable> = HashMap::new();
            
            let names = code_block.get_names(&self.refs);
            let consts = code_block.get_consts(&self.refs);
            let operations = code_block.get_operations();

            // Debug output of the read operations
            for op in operations {
                println!("{:?}: {:?}", code_block.get_name(&self.refs), op);
            }

            // First pass: Process operations using owned types, but defer return
            let mut return_var: Option<OwnedVariable> = None;
            let mut print_calls: Vec<(String, Vec<OwnedVariable>)> = Vec::new();
            
            for op in operations {
                match op {
                    Operation::LoadConstArg(i) => {
                        handlers.handle_load_const(&consts, *i, &mut variables, &mut stack);
                    }
                    Operation::StoreNameArg(i) => {
                        handlers.handle_store_name(&names, *i, &mut variables, &mut stack);
                    }
                    Operation::LoadNameArg(i) => {
                        handlers.handle_load_name(&names, *i, &variables, &mut stack);
                    }
                    Operation::BinaryAdd => {
                        handlers.handle_binary_add(&mut stack);
                    }
                    Operation::BinarySubtract => {
                        handlers.handle_binary_subtract(&mut stack);
                    }
                    Operation::ReturnValue => {
                        return_var = handlers.handle_return_value(&mut stack);
                    }
                    Operation::StopCode => {
                        // StopCode marks the end of bytecode - ignore it
                        continue;
                    }
                    Operation::PopTop => {
                        handlers.handle_pop_top(&mut stack);
                    }
                    Operation::CallFunctionArg(i) => {
                        // Handle function calls
                        let arg_count = *i;
                        
                        match handlers.handle_call_function(&names, arg_count, &variables, &mut stack) {
                            Ok((func_name, args)) => {
                                if builtins::is_builtin(&func_name) {
                                    match func_name.as_str() {
                                        builtins::PRINT => {
                                            // Store for later processing
                                            print_calls.push((func_name, args));
                                        }
                                        _ => {
                                            todo!("Builtin function '{}' not yet implemented", func_name);
                                        }
                                    }
                                } else {
                                    todo!("Function call to {} (not yet fully implemented)", func_name);
                                }
                            }
                            Err(e) => {
                                panic!("Function call error: {}", e);
                            }
                        }
                    }
                    _ => todo!("operation {:?}", op),
                }
            }

            // Second pass: Create LLVM allocations for stored variables
            for (name, owned_var) in &variables {
                if !owned_var.is_temp {
                    let llvm_type = owned_var.var_type.to_llvm_type(&context);
                    let llvm_value = owned_var.to_llvm_value(&context);
                    
                    let llvm_ptr = builder
                        .build_alloca(llvm_type, name)
                        .expect(&format!("expected llvm to create a local pointer for variable - {:?}", name));
                    
                    builder
                        .build_store(llvm_ptr, llvm_value)
                        .expect(&format!("llvm to store variable of type {:?}", llvm_type));
                    
                    let llvm_var = LlvmVariable {
                        ptr: BasicValueEnum::PointerValue(llvm_ptr),
                        v_type: llvm_type,
                        value: llvm_value,
                    };
                    
                    llvm_variables.insert(name.clone(), llvm_var);
                }
            }

            // Third pass: Handle deferred operations (print calls)
            for (func_name, args) in print_calls {
                match func_name.as_str() {
                    builtins::PRINT => {
                        if let Some(arg) = args.first() {
                            let llvm_arg = LlvmVariable {
                                v_type: arg.var_type.to_llvm_type(&context),
                                ptr: context.i32_type().const_zero().into(), // Placeholder
                                value: arg.to_llvm_value(&context),
                            };
                            builtins::handle_print_builtin(&builder, &module, &llvm_arg);
                        }
                    }
                    _ => {
                        todo!("Builtin function '{}' not yet implemented", func_name);
                    }
                }
            }

            // Finally: Handle return instruction
            if let Some(ret_var) = return_var {
                let llvm_value = ret_var.to_llvm_value(&context);
                let _ = builder.build_return(Some(&llvm_value));
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
