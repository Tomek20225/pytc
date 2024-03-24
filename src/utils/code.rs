use super::{operations::Operation, var::Var};

#[derive(Default, Debug, Clone)]
pub struct CodeBlock {
    pub co_argcount: i32,               // number of arguments (not including keyword only arguments, * or ** args)
    pub co_posonlyargcount: i32,        // number of positional only arguments
    pub co_kwonlyargcount: i32,         // number of keyword only arguments (not including ** arg)
    pub co_nlocals: i32,                // number of local variables
    pub co_stacksize: i32,              // virtual machine stack space required
    pub co_flags: i32,                  // bitmap of CO_* flags
    pub co_code: Vec<Operation>,        // string of raw compiled bytecode
    pub co_const: Box<Var>,             // tuple of constants used in the bytecode
    pub co_names: Box<Var>,             // tuple of names other than arguments and function locals
    pub co_varnames: Box<Var>,          // tuple of names of arguments and local variables
    pub co_freevars: Box<Var>,          // tuple of names of free variables (referenced via a function’s closure)
    pub co_cellvars: Box<Var>,          // tuple of names of cell variables (referenced by containing scopes)
    pub co_filename: Box<Var>,          // name of file in which this code object was created
    pub co_name: Box<Var>,              // name with which this code object was defined
    pub co_firstlineno: i32,            // number of first line in Python source code
    pub co_lnotab: Box<Var>,            // bytecode address-to-line information
    // pub co_exceptiontable: Box<Var>, // exception handling information
    // pub co_qualname: Box<Var>,       // fully qualified name with which this code object was defined
}

impl CodeBlock {
    pub fn get_name(&self, refs: &Vec<Var>) -> String {
        match &*self.co_name {
            Var::Ref(i) => {
                match &refs[*i as usize] {
                    Var::String(s) | Var::ShortAscii(s) | Var::ShortAsciiInterned(s) => s.clone(),
                    _ => panic!("co_name value is not a string")
                }
            },
            Var::String(s) | Var::ShortAscii(s) | Var::ShortAsciiInterned(s) => s.clone(),
            _ => panic!("co_name value is not a String or a Ref to a string")
        }
    }
}