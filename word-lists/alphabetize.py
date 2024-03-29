#!/usr/bin/env python3

# Run this script before committing a word list.
# Alphabetizing and stripping word lists will make updates more obvious.

import sys
from pathlib import Path

CURRENT_SCRIPT_DIR = sys.path[0]

print(f"Alphabetizing word lists in {CURRENT_SCRIPT_DIR}/*.txt")

for filename in Path(CURRENT_SCRIPT_DIR).glob('*.txt'):
    with open(filename, 'r') as f:
        words = f.read().splitlines()

    words.sort()

    with open(filename, 'w') as f:
        f.write(('\n'.join(words)).strip())