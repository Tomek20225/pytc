use inkwell::types::BasicTypeEnum;
use inkwell::builder::Builder;
use inkwell::module::Module;

use super::llvm::LlvmVariable;

// Global constants for supported builtin functions
pub const PRINT: &str = "print";
// Add more builtin functions here as needed
// pub const LEN: &str = "len";
// pub const RANGE: &str = "range";
// pub const INPUT: &str = "input";

/// Check if a function name is a supported builtin
pub fn is_builtin(name: &str) -> bool {
    matches!(name, PRINT)
}



/// Get the format string for the print function based on the argument type
pub fn get_print_format_string(arg_type: &BasicTypeEnum) -> &'static str {
    match arg_type {
        BasicTypeEnum::IntType(_) => "%d\n",
        _ => "%s\n", // Default to string representation for now
    }
}

/// Handle the print builtin function - now a regular function!
pub fn handle_print_builtin<'a>(
    builder: &'a Builder,
    module: &'a Module,
    arg: &LlvmVariable<'a>,
) {
    // Get the printf function from the module
    let printf_func = module.get_function("printf").expect("printf function should be declared");
    
    // Create format string for the value
    let format_str_text = get_print_format_string(&arg.v_type);
    let format_str = builder.build_global_string_ptr(format_str_text, "print_format")
        .expect("failed to create format string");
    
    // Call printf with the format string and the value
    let _ = builder.build_call(
        printf_func, 
        &[format_str.as_pointer_value().into(), arg.value.into()], 
        "print_result"
    );
    
    println!("DEBUG: Generated LLVM IR for print function call");
}
