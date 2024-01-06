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

fn process_code_block(reader: &mut Reader) -> CodeBlock {
    let mut code = CodeBlock{ ..Default::default() };

    // First byte
    let chunk_type = TypeIdentifier::from_byte(reader.get().unwrap()).unwrap();
    reader.next();
    // println!("type: {:?}", chunk_type);

    // Static params
    code.co_argcount = reader.read_long();
    code.co_kwonlyargcount = reader.read_long();
    code.co_nlocals = reader.read_long();
    code.co_posonlyargcount = reader.read_long();
    code.co_stacksize = reader.read_long();
    code.co_flags = *reader.get().unwrap();
    reader.jump(4);
    reader.next();
    code.co_code_size = *reader.get().unwrap();
    reader.jump(4);

    return code;
}

#[derive(Default)]
#[derive(Debug)]
struct Reader {
    current_idx: usize,
    contents: Vec<u8> 
}

impl Reader {
    fn get(&self) -> Option<&u8> {
        self.contents.get(self.current_idx)
    }

    fn get_by_idx(&self, idx: usize) -> Option<&u8> {
        self.contents.get(idx)
    }

    fn next(&mut self) {
        // TODO: EOF case
        self.current_idx += 1
    }

    fn jump(&mut self, jump: usize) {
        // TODO: EOF case
        self.current_idx += jump
    }

    fn is_eof(&self) -> bool {
        self.current_idx >= self.contents.len()
    }

    fn read_long(&mut self) -> i32 {
        let long = i32::from_le_bytes([
            self.contents[self.current_idx],
            self.contents[self.current_idx + 1],
            self.contents[self.current_idx + 2],
            self.contents[self.current_idx + 3],
        ]);
        self.jump(4);
        return long;
    }
}

fn main() {
    let filename = "./src/python/foo.pyc";
    let contents = fs::read(filename).expect("reading pyc file");
    let mut reader = Reader{
        current_idx: 0,
        contents: contents
    };
    let code = process_code_block(&mut reader);
    println!("{:?}", code);

    // for byte in contents.iter() {
    //     let ch = *byte as char;
    //     println!("{byte} {ch}");
    // }
}