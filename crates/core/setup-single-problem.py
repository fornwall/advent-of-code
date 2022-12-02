#!/usr/bin/env python3

import json
import os
import re
import sys
import shutil
import subprocess
from pathlib import Path

year = int(sys.argv[1])
day = int(sys.argv[2])
print(f"Setting up single problem for {year}-{day}")

filename = f"src/year{year}/day{day:02}.rs"
input_path = f"src/year{year}/day{day:02}_input.txt"
source_abs_path = os.path.abspath(filename)
input_abs_path =os.path.abspath(input_path)

dir_name = f"{Path.home()}/src/generated-single-problem-{year}-{day}"
shutil.rmtree(dir_name, ignore_errors=True)
os.makedirs(f'{dir_name}/src/year{year}')
os.makedirs(f'{dir_name}/benches')
with open(f'{dir_name}/src/lib.rs', 'w') as f:
    f.write(f"mod year{year};\nmod input;")
with open(f'{dir_name}/src/year{year}/mod.rs', 'w') as f:
    f.write(f"mod day{day:02};")
os.symlink(os.path.abspath("src/input.rs"), f'{dir_name}/src/input.rs')
os.symlink(source_abs_path, f'{dir_name}/src/year{year}/day{day:02}.rs')
os.symlink(input_abs_path, f'{dir_name}/src/year{year}/day{day:02}_input.txt')
crate_name = f"advent_of_code_{year}_{day}"
with open(f'{dir_name}/Cargo.toml', 'w') as f:
    f.write(f"""[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"
""")
print(dir_name)
