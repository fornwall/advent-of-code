#!/usr/bin/env python3

import json
import os
import re
import sys
from pathlib import Path

import requests

MAPPING_FILE_NAME = "gist-mapping.json"

dry_run = bool(os.environ.get('DRY_RUN'))

def add_header(src, year, day):
    link_to_file = f"https://github.com/fornwall/advent-of-code/tree/master/crates/core/src/year{year}/day{str(day).rjust(2, '0')}.rs"
    header = f"// Solution to Advent of Code {year}, day {day}: https://adventofcode.com/{year}/day/{day}"
    header += "\n//"
    header += "\n// This is the following file extracted into a gist for use in the Rust playground:"
    header += f"\n// {link_to_file}"
    header += "\n//"
    header += "\n// To suggest or discuss possible changes, open an issue or pull request at:"
    header += "\n// https://github.com/fornwall/advent-of-code"

    inlined_modules = set()
    pattern = re.compile(r"use super::(.*?)::")
    found = False
    for module in re.findall(pattern, src):
        if module in inlined_modules:
            continue
        inlined_modules.add(module)

        file_to_include = f"../../core/src/year{year}/{module}.rs"
        src_to_include = Path(file_to_include).read_text()
        header += f"\n\n#[allow(dead_code)]\nmod {module} {{\n"
        header += f"    // This is src/year{year}/{module}.rs inlined to work in the Rust Playground."
        for line in iter(src_to_include.splitlines()):
            if line:
                header += f"\n    {line}"
            else:
                header += "\n"
        header += "\n}"
        found = True

    src = re.sub(r"use super::(.*)?::", lambda match: f"use {match.group(1)}::", src)

    return header + "\n\n" + src


def replace_include_str(dirpath, src):
    def replace(match):
        included_file = match.group(1)
        replacement_file = os.path.join(dirpath, included_file)
        included_src = Path(replacement_file).read_text()
        included_src = included_src.replace("\\", "\\\\").replace("\n", "\\n")
        return f'"{included_src}"'

    return re.sub(r'include_str!\("(.*?)"\)', replace, src)


def set_gist(year, day, src, gist_id=None):
    API_TOKEN = os.environ["GIST_API_TOKEN"]

    file_name = f"year{year}_day{day}.rs"
    headers = {
        "Authorization": f"token {API_TOKEN}",
        "accept": "application/vnd.github.v3+json",
        "Content-Type": "application/json",
    }

    if gist_id:
        gist_method = "PATCH"
        GIST_API = f"https://api.github.com/gists/{gist_id}"
        get_response = requests.get(GIST_API, headers=headers)
        existing_src = get_response.json()["files"][file_name]["content"]
        if existing_src == src:
            print("Unmodified")
            return gist_id
    else:
        gist_method = "POST"
        GIST_API = "https://api.github.com/gists"

    payload = {
        "description": f"Solution for Advent of Code: Year {year}, day {day}.",
        "files": {file_name: {"content": src}},
    }

    if not gist_id:
        payload["public"] = False

    response = requests.request(gist_method, GIST_API, headers=headers, json=payload)
    return response.json()["id"]


with open(MAPPING_FILE_NAME, "r") as infile:
    gist_mapping = json.load(infile)

for (dirpath, dirnames, filenames) in os.walk("../../core/src/"):
    if not 'year' in dirpath:
        continue
    year = int(dirpath.split('/')[-1][4:])
    for filename in filenames:
        if not (filename.endswith('.rs') and filename.startswith('day')):
            continue
        day = int(filename[3:][:-3])
        path = os.path.join(dirpath, filename)

        print(f"{year} - {day}")

        src = Path(path).read_text()
        src = add_header(src, year, day)
        src = replace_include_str(dirpath, src)

        year_str = str(year)
        day_str = str(day)

        if year_str in gist_mapping and day_str in gist_mapping[year_str]:
            existing_id = gist_mapping[year_str][day_str]
            if dry_run:
                print(f'Would reuse existing id {existing_id}');
            else:
                set_gist(year, day, src, existing_id)
        else:
            if dry_run:
                print('Would create new!');
            else:
                new_id = set_gist(year, day, src)
                if year_str not in gist_mapping:
                    gist_mapping[year_str] = {}
                gist_mapping[year_str][day_str] = new_id

if not dry_run:
    with open(MAPPING_FILE_NAME, "w") as outfile:
        json.dump(gist_mapping, outfile, indent=2)
