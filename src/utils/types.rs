const FLAG_REF: u8 = 0x80; // with a type, add obj to index

// TODO: Finish translating types into an enum

#[derive(Debug)]
pub enum TypeIdentifier {
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
    pub fn from_byte(byte: &u8) -> Option<Self> {
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

    // pub fn size(&self) -> usize {
    //     match self {
    //         TypeIdentifier::TypeA => 2,
    //         TypeIdentifier::TypeB => 4,
    //         TypeIdentifier::TypeC => 1,
    //         // ... other types
    //     }
    // }

    pub fn parse_value() {
        todo!()
    }
}

// fn read_type(byte: &u8) -> Option<TypeIdentifier> {
//     TypeIdentifier::from_byte(byte)
// }