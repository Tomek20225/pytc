use super::{reader::CodeBlock, reader::Reader};

const FLAG_REF: u8 = 0x80; // with a type, add obj to index

// TODO: Finish translating types into an enum
#[derive(Debug, Default)]
pub enum Var {
    #[default]
    Null,                       // 0
    None,                       // N
    True,                       // T
    False,                      // F
    Int(i32),                   // i
    Long(i32),                  // l
    Code(CodeBlock),            // c
    Ref(u32),                   // r - seems to be an address
    FlagRef(Box<Var>),          // '\x80' with a type, points to external refs vector, used to determine whether a serialized object should be tracked for potential future references within the serialized data stream
    String(String),             // s, also used for coded objects
    ShortAscii(String),         // \xfa (250) or z
    ShortAsciiInterned(String), // \xda (218) or Z
    SmallTuple(Vec<Var>),       // )

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
        if byte & FLAG_REF != 0 {
            // FLAG_REF is set, extract the type it references from lower 7 bits
            Some(Var::FlagRef(Box::new(Var::from_byte(
                &(byte & 0x7F),
                reader,
            )?)))
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
                _ => None,
            }
        }
    }
}
