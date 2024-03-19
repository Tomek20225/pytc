import opcode
from re import sub

def camel_case(s):
    s = sub(r"(_|-)+", " ", s).title().replace(" ", "")
    return ''.join([s[0], s[1:]])

print(opcode.opmap)

FILE = open('./opcodes', 'w')

for key in opcode.opmap:
    val = opcode.opmap[key]
    key = camel_case(key)
    # FILE.write(f"{key}(u8),\n")
    FILE.write(f"{val} => Some(Operation::{key}(next_byte)),\n")