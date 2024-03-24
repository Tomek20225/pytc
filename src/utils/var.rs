use super::{code::CodeBlock, reader::Reader};
use inkwell::{
    context::Context,
    types::{BasicType, BasicTypeEnum},
};
use std::str;

const FLAG_REF: u8 = 0x80; // with a type, add obj to index

// TODO: Finish translating types into an enum
#[derive(Debug, Default, Clone)]
pub enum Var {
    #[default]
    Null, // 0
    None,                       // N
    True,                       // T
    False,                      // F
    Int(i32),                   // i
    Long(i32),                  // l
    Code(CodeBlock),            // c
    Ref(u32),                   // r - seems to be an address
    FlagRef(Box<Var>), // '\x80' with a type, points to external refs vector, used to determine whether a serialized object should be tracked for potential future references within the serialized data stream
    String(String),    // s, also used for coded objects
    ShortAscii(String), // \xfa (250) or z
    ShortAsciiInterned(String), // \xda (218) or Z
    SmallTuple(Vec<Var>), // )

                       // STOPITER           'S'
                       // ELLIPSIS           '.'
                       // INT64              'I'
                       // FLOAT              'f'
                       // BINARY_FLOAT       'g'
                       // COMPLEX            'x'
                       // BINARY_COMPLEX     'y'
                       // STRING             's'
                       // INTERNED           't'
                       // TUPLE              '('
                       // LIST               '['
                       // DICT               '{'
                       // UNICODE            'u'
                       // UNKNOWN            '?'
                       // SET                '<'
                       // FROZENSET          '>'
                       // ASCII              'a'
                       // ASCII_INTERNED     'A'
                       // SMALL_TUPLE        ')'
                       // SHORT_ASCII_INTERNED 'Z'
                       // WFERR_OK 0
                       // WFERR_UNMARSHALLABLE 1
                       // WFERR_NESTEDTOODEEP 2
                       // WFERR_NOMEMORY 3
}

impl Var {
    pub fn from_byte(byte: &u8, reader: &mut Reader) -> Option<Self> {
        // FLAG_REF is set
        if byte & FLAG_REF != 0 {
            // Get current amount of refs
            // If there is one, it means it's the main CodeBlock
            // In that case returning Var::Ref shouldn't happen
            // TODO: Replace this dirty fix with something appropriate
            let is_main_code_block = reader.get_refs_len() == 0;
            if is_main_code_block {
                reader.push_ref(Var::None);
            }

            // Extract the type the flag references from lower 7 bits
            let var = Var::from_byte(&(byte & 0x7F), reader)?;

            // Push the var to refs vector and get the index of last element
            // if it isn't the main CodeBlock
            //
            // This is backwards from original CPython implementation,
            // as it stores the var in a vector and then returns a Ref
            // instead of returning the var wrapped in FlagRef and pushing the reference to the refs vector
            if !is_main_code_block {
                let idx = reader.push_ref(var);
                return Some(Var::Ref(
                    idx.try_into()
                        .expect("usize in Var:from_byte to convert to u32"),
                ));
            }

            // Return the var, as it is the main CodeBlock
            Some(var)
        } else {
            match byte {
                b'0' => Some(Var::Null),
                b'N' => Some(Var::None),
                b'T' => Some(Var::True),
                b'F' => Some(Var::False),
                b'i' => Some(Var::Int(reader.read_int())),
                b'l' => Some(Var::Long(reader.read_long())),
                b'c' => Some(Var::Code(reader.read_code())),
                &b'r' => Some(Var::Ref(reader.read_ulong())),
                &b's' => Some(Var::String(reader.read_string())),
                0xfa | b'z' => Some(Var::ShortAscii(reader.read_short_string())), // TODO: Check why this gets caught by FlagRef and if it should
                0xda | b'Z' => Some(Var::ShortAsciiInterned(reader.read_short_string())), // TODO: Check why this gets caught by FlagRef and if it should
                &b')' => Some(Var::SmallTuple(reader.read_tuple())),
                _ => {
                    let bytes = vec![*byte];
                    todo!("type {} (value {})", str::from_utf8(&bytes).unwrap(), byte)
                } // _ => None,
            }
        }
    }

    pub fn get_type<'a>(&'a self, ctx: &'a Context) -> BasicTypeEnum {
        match self {
            Var::Int(_) => ctx.i32_type().as_basic_type_enum(),
            _ => todo!("can't get type of var {:?}", self),
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        if let Var::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Var::String(s) | Var::ShortAscii(s) | Var::ShortAsciiInterned(s) => Some(s.clone()),
            _ => None,
        }
    }
}
