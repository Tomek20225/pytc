import sys
import subprocess
import os

TRANSLATION = {
    'print': 'printf',
    '(': '(',
    ')': ')',
    '"': '"',
    "'": '"',
}

def tokenize(line: str) -> list[str]:
    found_tokens = []
    found_string = ""

    i = 0
    while i < len(line):
        found_token = False
        for token in TRANSLATION:
            if line[i:].startswith(token):
                if found_string != "":
                    found_tokens.append(found_string)
                    found_string = ""
                found_tokens.append(TRANSLATION[token])
                i += len(token) - 1
                found_token = True
                break
        if not found_token:
            found_string += line[i]
        i += 1
    
    found_tokens.append(';')
    return found_tokens

def create_c(headers: list[str], tokens: list[str], start: str = "int main() {", end: str = ";return 0;}") -> list[str]:
    lines = []
    for header in headers:
        lines.append(f"#include <{header}>")
    lines.append(start)
    lines.append("".join(tokens))
    lines.append(end)
    return lines


PY_FILE_NAME = sys.argv[1][:-3]
with open(f'./{PY_FILE_NAME}.py', "r") as PY_FILE:
    LINES = [line.strip() for line in PY_FILE if line.strip()]

    tokens = []
    for line in LINES:
        tokens += tokenize(line)

    c_headers = ["stdio.h"]
    c_lines = create_c(c_headers, tokens)
    c_filename = f'./{PY_FILE_NAME}.c'

    with open(c_filename, "w") as OUT_FILE:
        for line in c_lines:
            OUT_FILE.write(line + '\n')
    
    subprocess.run(['gcc', '-o', PY_FILE_NAME, c_filename])
    os.remove(c_filename)
