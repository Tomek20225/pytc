use super::{operations::Operation, var::Var};
use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum};

#[derive(Default, Debug, Clone)]
pub struct CodeBlock {
    pub co_argcount: i32, // number of arguments (not including keyword only arguments, * or ** args)
    pub co_posonlyargcount: i32, // number of positional only arguments
    pub co_kwonlyargcount: i32, // number of keyword only arguments (not including ** arg)
    pub co_nlocals: i32,  // number of local variables
    pub co_stacksize: i32, // virtual machine stack space required
    pub co_flags: i32,    // bitmap of CO_* flags
    pub co_code: Vec<Operation>, // string of raw compiled bytecode
    pub co_const: Box<Var>, // tuple of constants used in the bytecode
    pub co_names: Box<Var>, // tuple of names other than arguments and function locals
    pub co_varnames: Box<Var>, // tuple of names of arguments and local variables
    pub co_freevars: Box<Var>, // tuple of names of free variables (referenced via a function’s closure)
    pub co_cellvars: Box<Var>, // tuple of names of cell variables (referenced by containing scopes)
    pub co_filename: Box<Var>, // name of file in which this code object was created
    pub co_name: Box<Var>,     // name with which this code object was defined
    pub co_firstlineno: i32,   // number of first line in Python source code
    pub co_lnotab: Box<Var>,   // bytecode address-to-line information
                               // pub co_exceptiontable: Box<Var>, // exception handling information
                               // pub co_qualname: Box<Var>,       // fully qualified name with which this code object was defined
}

impl CodeBlock {
    pub fn get_name(&self, refs: &Vec<Var>) -> String {
        match &*self.co_name {
            Var::Ref(i) => match &refs[*i as usize] {
                Var::String(s) | Var::ShortAscii(s) | Var::ShortAsciiInterned(s) => s.clone(),
                _ => panic!("co_name value is not a string"),
            },
            Var::String(s) | Var::ShortAscii(s) | Var::ShortAsciiInterned(s) => s.clone(),
            _ => panic!("co_name value is not a String or a Ref to a string"),
        }
    }

    fn get_deref_vec<'a>(&'a self, refs: &'a Vec<Var>, vec: &'a Box<Var>) -> Vec<&Var> {
        let mut vars: Vec<&Var> = Vec::new();
        if let Var::SmallTuple(vars_temp) = &**vec {
            for var in vars_temp {
                match var {
                    Var::Ref(i) => vars.push(&refs[*i as usize]),
                    _ => vars.push(var),
                }
            }
            vars
        } else {
            panic!("{:?} is not a Tuple", vec)
        }
    }

    pub fn get_consts<'a>(&'a self, refs: &'a Vec<Var>) -> Vec<&Var> {
        self.get_deref_vec(refs, &self.co_const)
    }

    pub fn get_names<'a>(&'a self, refs: &'a Vec<Var>) -> Vec<String> {
        let names_as_vars = self.get_deref_vec(refs, &self.co_names);
        let mut names: Vec<String> = vec![];
        for var in names_as_vars {
            match var {
                Var::String(s) | Var::ShortAscii(s) | Var::ShortAsciiInterned(s) => {
                    names.push(s.clone())
                }
                _ => panic!("Found non-string value in co_names vector"),
            }
        }
        names
    }

    pub fn get_code_blocks<'a>(&'a self, refs: &'a Vec<Var>) -> Vec<&CodeBlock> {
        let mut code_blocks: Vec<&CodeBlock> = vec![self];
        for const_var in self.get_consts(refs) {
            match const_var {
                Var::Code(c) => code_blocks.push(&c),
                _ => continue,
            }
        }
        code_blocks
    }

    pub fn get_operations(&self) -> &Vec<Operation> {
        &self.co_code
    }

    pub fn get_op_type<'a>(
        &'a self,
        refs: &'a Vec<Var>,
        ctx: &'a Context,
        operation: &'a Operation,
    ) -> BasicTypeEnum {
        match operation {
            Operation::LoadConst(i) => {
                let consts = self.get_consts(refs);
                let var = consts[*i as usize];
                match var {
                    Var::None => ctx.i32_type().as_basic_type_enum(),
                    Var::Int(_) | Var::Long(_) => ctx.i32_type().as_basic_type_enum(),
                    _ => todo!("{:?} as return value", var),
                }
            }
            _ => todo!("operation {:?} as return value", operation),
        }
    }

    // TODO: This may cause problems in case of more complex programs
    // Because the return type is the type of the Var on top of the stack
    pub fn get_return_type<'a>(&'a self, refs: &'a Vec<Var>, ctx: &'a Context) -> BasicTypeEnum {
        let operations = self.get_operations();
        let ret_op = &operations[operations.len() - 2];
        self.get_op_type(refs, ctx, ret_op)
    }
}
