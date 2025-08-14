use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::BasicValueEnum;
use std::collections::HashMap;

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

/// Create a placeholder variable for builtin functions during load_name
pub fn create_builtin_placeholder<'a>(context: &'a Context) -> LlvmVariable<'a> {
    LlvmVariable {
        v_type: context.i32_type().as_basic_type_enum(), // Placeholder type
        ptr: BasicValueEnum::IntValue(context.i32_type().const_int(0, false)), // Placeholder pointer
        value: BasicValueEnum::IntValue(context.i32_type().const_int(0, false)), // Placeholder value
    }
}

/// Find a function name in the names list, checking both variables and builtins
pub fn find_function_name<'a>(
    names: &[String],
    variables_ptr: &HashMap<String, LlvmVariable<'a>>,
    func_name_var: &LlvmVariable<'a>,
) -> Option<String> {
    names.iter().find(|name| {
        if let Some(var) = variables_ptr.get(*name) {
            var.ptr == func_name_var.ptr
        } else {
            // Check if it's a built-in function
            is_builtin(name)
        }
    }).cloned()
}

/// Get the format string for the print function based on the argument type
pub fn get_print_format_string(arg_type: &BasicTypeEnum) -> &'static str {
    match arg_type {
        BasicTypeEnum::IntType(_) => "%d\n",
        _ => "%s\n", // Default to string representation for now
    }
}

/// Handle the print builtin function - contains the implementation logic
/// This is a macro to avoid lifetime issues while keeping the logic organized
#[macro_export]
macro_rules! handle_print_builtin {
    ($builder:expr, $module:expr, $arg:expr) => {{
        // Get the printf function from the module
        let printf_func = $module.get_function("printf").expect("printf function should be declared");
        
        // Create format string for the value
        let format_str_text = $crate::utils::builtins::get_print_format_string(&$arg.v_type);
        let format_str = $builder.build_global_string_ptr(format_str_text, "print_format")
            .expect("failed to create format string");
        
        // Call printf with the format string and the value
        let _ = $builder.build_call(
            printf_func, 
            &[format_str.as_pointer_value().into(), $arg.value.into()], 
            "print_result"
        );
        
        println!("DEBUG: Generated LLVM IR for print function call");
    }};
}
