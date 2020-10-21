#!/usr/bin/env python3

import sys

print("```diff")
first_line = True
for line in open(sys.argv[1]):
    line = line.strip()
    if first_line:
        first_line = False
        print("@@" + "Benchmark Diff".center(len(line) - 2) + "@@")
        start = "# "
    else:
        speedup = float(line.split(" ")[-1])
        delta = 0.01
        if speedup < 1 - delta:
            start = "- "
        elif speedup > 1 + delta:
            start = "+ "
        else:
            start = "  "

    print(start + line)
print("```")
