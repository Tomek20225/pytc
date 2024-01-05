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
    // FLAG_REF            '\x80' /* with a type, add obj to index */
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

fn read_long(contents: &Vec<u8>, start_idx: usize) -> i32 {
    let long_int = i32::from_le_bytes([
                            contents[start_idx],
                            contents[start_idx + 1],
                            contents[start_idx + 2],
                            contents[start_idx + 3],
                        ]);
    return long_int;
}

fn read_type(byte: &u8) -> Option<TypeIdentifier> {
    TypeIdentifier::from_byte(byte)
}

fn main() {
    let filename = "./src/python/foo.pyc";
    let contents = fs::read(filename).expect("reading pyc file");
    let mut i = 0;

    // First byte
    let chunk_type = TypeIdentifier::from_byte(&contents[i]).unwrap();
    println!("type: {:?}", chunk_type);
    i += 1;

    // Next 4 bytes
    let co_argcount = read_long(&contents, i);
    println!("co_argcount: {:?}", co_argcount);
    i += 4;

    // Next 4 bytes
    let co_kwonlyargcount = read_long(&contents, i);
    println!("co_kwonlyargcount: {:?}", co_kwonlyargcount);
    i += 4;

    // Next 4 bytes
    let co_nlocals = read_long(&contents, i);
    println!("co_nlocals: {:?}", co_nlocals);
    i += 4;

    // Next 4 bytes
    let co_posonlyargcount = read_long(&contents, i);
    println!("co_posonlyargcount: {:?}", co_posonlyargcount);
    i += 4;

    // Next 4 bytes
    let co_stacksize = read_long(&contents, i);
    println!("co_stacksize: {:?}", co_stacksize);
    i += 4;

    // for byte in contents.iter() {
    //     let ch = *byte as char;
    //     println!("{byte} {ch}");
    // }
}