#!/usr/bin/env python3

import sys
import os
import glob
import json
import subprocess

if len(sys.argv) == 2:
    FILTER = "problem_" + sys.argv[1] + "_"
else:
    FILTER = None

bench_name = "iai"
output_file = open(f"benches/{bench_name}.rs", "w")

print(
    """use advent_of_code::solve;
use std::fs::read_to_string;

""",
    file=output_file,
)

output_dict = {}
main_string = ""

for year in range(2024, 2025):
    for day in range(1, 26):
        for part in range(1, 2 if day == 25 else 3):
            problem_func = f"problem_{year}_{day}_{part}"
            if FILTER and not problem_func.startswith(FILTER):
                continue

            with open(f"src/year{year}/day{day:02}.rs", "r") as f:
                src = f.read()
                if "pub const fn solve" in src:
                    output_dict[f"{year}_{day}_{part}"] = {
                        "year": year,
                        "day": day,
                        "part": part,
                        "instructions": 10_000_000,
                    }
                    continue

            problem_src = f"""fn {problem_func}() {{
            #![allow(clippy::unwrap_used)]
  let input = read_to_string("src/year{year}/day{day:02}_input.txt").unwrap();
  solve({year}, {day}, {part}, &input).unwrap();
}}
"""
            print(problem_src, file=output_file)
            if main_string:
                main_string += ", "
            else:
                main_string = "iai::main!("
            main_string += problem_func

main_string += ");"
print(main_string, file=output_file)
output_file.close()

subprocess.run(["cargo", "fmt"], check=True)

result = subprocess.run(
    ["cargo", "bench", "--bench", bench_name],
    check=True,
    stdout=subprocess.PIPE,
    text=True,
)
# print(result.stdout)
current_year = None
current_day = None
current_part = None
current = {}
for line in result.stdout.splitlines():
    line = line.strip()
    if line.startswith("problem"):
        string_parts = line.split("_")
        current_year = int(string_parts[1])
        current_day = int(string_parts[2])
        current_part = int(string_parts[3])
        current = {"year": current_year, "day": current_day, "part": current_part}
        output_dict[line[len("problem_") :]] = current
    else:
        words = [word for word in line.split(" ") if word]
        if line.startswith("Instructions:"):
            current["instructions"] = int(words[1])

print(json.dumps(output_dict, indent=2))
