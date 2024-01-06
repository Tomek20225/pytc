use super::types::TypeIdentifier;
use super::reader::Reader;

#[derive(Default)]
#[derive(Debug)]
pub struct CodeBlock {
    pub co_argcount: i32, // Number of positional arguments
    pub co_kwonlyargcount: i32, // Number of keyworded arguments
    pub co_nlocals: i32, // Number of local variables, including args and kwargs
    pub co_posonlyargcount: i32,
    pub co_stacksize: i32,
    pub co_flags: i32,
    pub co_code_size: i32,
    pub co_code: Vec<TypeIdentifier>,
    pub co_const: Vec<u8>, // tbd
    pub co_names: Vec<String>, // tbd,
    pub co_varnames: Vec<u8>, // tbd
    pub co_freevars: Vec<u8>, // tbd
    pub co_cellvars: Vec<u8>, // tbd
    pub co_filename: Vec<u8>, // tbd
    pub co_name: String, // tbd
    pub co_firstlineno: i32,
    pub co_lnotab: Vec<u8>, // tbd
}

pub fn process_code_block(reader: &mut Reader) -> CodeBlock {
    let mut code = CodeBlock{ ..Default::default() };

    // First byte - represents the FlagReference to the Code chunk
    // TypeIdentifier::from_byte(reader.get().unwrap()).unwrap();
    reader.next();

    // Static params
    code.co_argcount = reader.read_long();
    code.co_kwonlyargcount = reader.read_long();
    code.co_nlocals = reader.read_long();
    code.co_posonlyargcount = reader.read_long();
    code.co_stacksize = reader.read_long();
    code.co_flags = reader.read_long();
    reader.next(); // skip 1 byte flag representing the co_code_size
    code.co_code_size = reader.read_long();

    // Instructions (next co_code_size bytes)
    // for i in 0..46 {
    //     println!("{i}");
    // }

    return code;
}