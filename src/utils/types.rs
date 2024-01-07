use super::{processing::CodeBlock, reader::Reader};

const FLAG_REF: u8 = 0x80; // with a type, add obj to index

// TODO: Finish translating types into an enum
#[derive(Debug)]
pub enum TypeIdentifier {
    None,                         // N
    True,                         // T
    Int(i32),                     // i
    Long(i32),                    // l
    Code(CodeBlock),              // c
    Ref(u32),                     // r - seems to be an address
    FlagRef(Box<TypeIdentifier>), // '\x80' with a type
    ShortAsciiInterned(String),   // \xda (218) or Z    ? probably, for now treated as a string
    SmallTuple,                   // )

                                  // NULL               '0'
                                  // NONE               'N'
                                  // FALSE              'F'
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
                                  // SHORT_ASCII        'z'
                                  // SHORT_ASCII_INTERNED 'Z'
                                  // WFERR_OK 0
                                  // WFERR_UNMARSHALLABLE 1
                                  // WFERR_NESTEDTOODEEP 2
                                  // WFERR_NOMEMORY 3
}

impl TypeIdentifier {
    pub fn from_byte(byte: &u8, reader: &mut Reader) -> Option<Self> {
        if byte & FLAG_REF != 0 {
            // FLAG_REF is set, extract the type it references from lower 7 bits
            Some(TypeIdentifier::FlagRef(Box::new(
                TypeIdentifier::from_byte(&(byte & 0x7F), reader)?,
            )))
        } else {
            match byte {
                b'N' => Some(TypeIdentifier::None),
                b'T' => Some(TypeIdentifier::True),
                b'i' => Some(TypeIdentifier::Int(reader.read_int())),
                b'l' => Some(TypeIdentifier::Long(reader.read_long())),
                b'c' => Some(TypeIdentifier::Code(CodeBlock {
                    ..Default::default()
                })),
                &b'r' => Some(TypeIdentifier::Ref(reader.read_ulong())),
                0xda => Some(TypeIdentifier::ShortAsciiInterned(reader.read_string())), // TODO: Check why this gets caught by FlagRef and if it should
                b'Z' => Some(TypeIdentifier::ShortAsciiInterned(reader.read_string())),
                &b')' => Some(TypeIdentifier::SmallTuple), // TODO: Make it accept the whole Vec?
                _ => None,
            }
        }
    }
}
