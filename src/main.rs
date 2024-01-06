use std::fs;

const FLAG_REF: u8 = 0x80; // with a type, add obj to index

#[derive(Debug)]
enum TypeIdentifier {
    // Single Type
    None, // N
    True, // T

    // Short Type
    ShortAsciiInterned(u8),
    String(u8),
    Long, // l

    Code, // c

    Ref, // r
    FlagRef(u8) // '\x80' with a type

    // NULL               '0'
    // NONE               'N'
    // FALSE              'F'
    // TRUE               'T'
    // STOPITER           'S'
    // ELLIPSIS           '.'
    // INT                'i'
    // INT64              'I'
    // FLOAT              'f'
    // BINARY_FLOAT       'g'
    // COMPLEX            'x'
    // BINARY_COMPLEX     'y'
    // STRING             's'
    // INTERNED           't'
    // REF                'r'
    // TUPLE              '('
    // LIST               '['
    // DICT               '{'
    // CODE               'c'
    // UNICODE            'u'
    // UNKNOWN            '?'
    // SET                '<'
    // FROZENSET          '>'
    // ASCII              'a'
    // ASCII_INTERNED     'A'
    // SMALL_TUPLE        ')'
    // SHORT_ASCII        'z'
    // SHORT_ASCII_INTERNED 'Z'
    // WFERR_OK 0
    // WFERR_UNMARSHALLABLE 1
    // WFERR_NESTEDTOODEEP 2
    // WFERR_NOMEMORY 3
}

impl TypeIdentifier {
    fn from_byte(byte: &u8) -> Option<Self> {
        if byte & FLAG_REF != 0 {
            // FLAG_REF is set, extract the type it references from lower 7 bits
            Some(TypeIdentifier::FlagRef(byte & 0x7F))
        }
        else {
            match byte {
                b'N' => Some(TypeIdentifier::None),
                b'T' => Some(TypeIdentifier::True),
                b'l' => Some(TypeIdentifier::Long),
                b'c' => Some(TypeIdentifier::Code),
                &b'r' => Some(TypeIdentifier::Ref),
                _ => None
            }
        }
    }

    // fn size(&self) -> usize {
    //     match self {
    //         TypeIdentifier::TypeA => 2,
    //         TypeIdentifier::TypeB => 4,
    //         TypeIdentifier::TypeC => 1,
    //         // ... other types
    //     }
    // }

    fn parse_value() {
        todo!()
    }
}

fn read_long(binary: &Vec<u8>, start_idx: usize) -> i32 {
    let long_int = i32::from_le_bytes([
        binary[start_idx],
        binary[start_idx + 1],
        binary[start_idx + 2],
        binary[start_idx + 3],
    ]);
    return long_int;
}

fn read_type(byte: &u8) -> Option<TypeIdentifier> {
    TypeIdentifier::from_byte(byte)
}

#[derive(Default)]
#[derive(Debug)]
struct CodeBlock {
    co_argcount: i32,
    co_kwonlyargcount: i32,
    co_nlocals: i32,
    co_posonlyargcount: i32,
    co_stacksize: i32,
    co_flags: u8,
    co_code_size: u8, // possibly too small
    co_code: Vec<TypeIdentifier>,
    co_const: Vec<u8>, // tbd
    co_names: Vec<String>, // tbd,
    co_varnames: Vec<u8>, // tbd
    co_freevars: Vec<u8>, // tbd
    co_cellvars: Vec<u8>, // tbd
    co_filename: Vec<u8>, // tbd
    co_name: String, // tbd
    co_firstlineno: u8, // tbd
    co_lnotab: Vec<u8>, // tbd
}

fn process_code_block(binary: &Vec<u8>) -> CodeBlock {
    let mut code = CodeBlock{ ..Default::default() };
    let mut i = 0;

    // First byte
    let chunk_type = TypeIdentifier::from_byte(&binary[i]).unwrap();
    // println!("type: {:?}", chunk_type);
    i += 1;

    // Static params
    code.co_argcount = read_long(&binary, i);
    i += 4;
    code.co_kwonlyargcount = read_long(&binary, i);
    i += 4;
    code.co_nlocals = read_long(&binary, i);
    i += 4;
    code.co_posonlyargcount = read_long(&binary, i);
    i += 4;
    code.co_stacksize = read_long(&binary, i);
    i += 4;
    code.co_flags = binary[i];
    i += 4;
    i += 1;
    code.co_code_size = binary[i];
    i += 4;

    return code;
}

fn main() {
    let filename = "./src/python/foo.pyc";
    let contents = fs::read(filename).expect("reading pyc file");
    let code = process_code_block(&contents);
    println!("{:?}", code);

    // for byte in contents.iter() {
    //     let ch = *byte as char;
    //     println!("{byte} {ch}");
    // }
}