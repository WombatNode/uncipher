#!/usr/bin/env python3

import sys
from collections import Counter

letters = Counter()

with open(sys.argv[1]) as fp:
    for c in fp.read():
        if c.isalpha():
            letters[c] += 1

for (letter, freq) in sorted(letters.items()):
    print(letter, freq)