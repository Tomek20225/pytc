use super::operations::Operation;
use super::reader::Reader;
use super::types::TypeIdentifier;

#[derive(Default, Debug)]
pub struct CodeBlock {
    pub co_argcount: i32,       // Number of positional arguments
    pub co_kwonlyargcount: i32, // Number of keyworded arguments
    pub co_nlocals: i32,        // Number of local variables, including args and kwargs
    pub co_posonlyargcount: i32,
    pub co_stacksize: i32,
    pub co_flags: i32,
    pub co_code_size: i32,
    pub co_code: Vec<Operation>,
    pub co_const: Vec<TypeIdentifier>,
    pub co_names: Vec<String>, // tbd,
    pub co_varnames: Vec<u8>,  // tbd
    pub co_freevars: Vec<u8>,  // tbd
    pub co_cellvars: Vec<u8>,  // tbd
    pub co_filename: Vec<u8>,  // tbd
    pub co_name: String,       // tbd
    pub co_firstlineno: i32,
    pub co_lnotab: Vec<u8>, // tbd
}

pub fn process_code_block(reader: &mut Reader) -> CodeBlock {
    let mut code = CodeBlock {
        ..Default::default()
    };

    // First byte - represents the FlagReference to the Code chunk
    // TypeIdentifier::from_byte(&reader.read_byte()).expect("reading first byte");
    reader.next();

    // Static params
    code.co_argcount = reader.read_long();
    code.co_kwonlyargcount = reader.read_long();
    code.co_nlocals = reader.read_long();
    code.co_posonlyargcount = reader.read_long();
    code.co_stacksize = reader.read_long();
    code.co_flags = reader.read_long();
    reader.next(); // skip 1 byte flag representing the co_code_size, 's'
    code.co_code_size = reader.read_long();

    // Instructions (next co_code_size bytes)
    let mut co_code: Vec<Operation> = Vec::new();
    let limit = reader.get_current_idx() + code.co_code_size as usize;
    while reader.get_current_idx() < limit {
        let instruction = reader.read_instruction().unwrap_or_else(|| {
            panic!("reading instruction from byte {}", reader.get_current_idx())
        });
        co_code.push(instruction);
    }
    code.co_code = co_code;

    // TODO: co_consts - each const can be a CodeBlock
    // Skip 1 byte representing start of a co_consts tuple
    // TypeIdentifier::from_byte(&reader.read_byte()).expect("reading co_const size");
    reader.next();
    let co_const_size = reader.read_byte();

    let mut co_const: Vec<TypeIdentifier> = Vec::new();
    // TODO: Process vars in a dynamic loop (5 consts are in the example foo.py file)
    // TODO: Process code consts properly (can be done after finishing of this function)
    co_const.push(reader.read_var().expect("reading first const"));
    co_const.push(reader.read_var().expect("reading second const"));
    co_const.push(reader.read_var().expect("reading third const"));
    reader.jump(86); // size of the code chunk in third const in foo.py file
    co_const.push(reader.read_var().expect("reading fourth const"));
    co_const.push(reader.read_var().expect("reading fifth const"));
    code.co_const = co_const;

    // TODO: co_names
    // TODO: co_varnames
    // TODO: co_freevars
    // TODO: co_cellvars
    // TODO: co_filename
    // TODO: co_name
    // TODO: co_firstlineno
    // TODO: co_lnotab

    code
}
