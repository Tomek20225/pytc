import marshal, types, dis

# Marshalling
source_py = "./foo.py"
source_name = source_py[2:-3]

with open(source_py) as f_source:
    source_code = f_source.read()

code_obj_compile = compile(source_code, source_py, "exec")

data = marshal.dumps(code_obj_compile)

# print(data)
out_pyc = f"./{source_name}.pyc"
with open(out_pyc, 'wb') as f_out:
    f_out.write(data)


# Disassembly
dis.dis(code_obj_compile)

for x in code_obj_compile.co_consts:
    if isinstance(x, types.CodeType):
        sub_byte_code = x
        func_name = sub_byte_code.co_name
        print('\nDisassembly of %s:' % func_name)
        dis.dis(sub_byte_code)

def print_co_obj_fields(code_obj):
    # Iterating through all instance attributes
    # and calling all having the 'co_' prefix
    for name in dir(code_obj):
        if name.startswith('co_'):
            co_field = getattr(code_obj, name)
            print(f'{name:<20} = {co_field}')

print_co_obj_fields(code_obj_compile)