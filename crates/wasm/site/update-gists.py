#!/usr/bin/env python3

import json
import os
import re
import sys
import subprocess
from pathlib import Path

import requests

MAPPING_FILE_NAME = "gist-mapping.json"

dry_run = bool(os.environ.get("DRY_RUN"))

def add_header(src, year, day):
    link_to_file = f"https://github.com/fornwall/advent-of-code/tree/master/crates/core/src/year{year}/day{str(day).rjust(2, '0')}.rs"
    header = f"// Solution to Advent of Code {year}, day {day}: https://adventofcode.com/{year}/day/{day}"
    header += "\n//"
    header += "\n// This is the following file extracted into a gist for use in the Rust playground:"
    header += f"\n// {link_to_file}"
    header += "\n//"
    header += (
        "\n// To suggest or discuss possible changes, open an issue or pull request at:"
    )
    header += "\n// https://github.com/fornwall/advent-of-code"

    # Put inlined modules last as they're not relevant compared to the solution:
    suffix = "\n"

    inlined_modules = set()
    pattern = re.compile(r"use (super|crate)::(.*)::(.*?);")
    found = False
    for crate_or_super, module, _ in re.findall(pattern, src):
        if module in inlined_modules:
            continue
        inlined_modules.add(module)

        module_path = module.replace("::", "/")
        if crate_or_super == "super":
            path_in_repo = f"crates/core/src/year{year}/{module_path}.rs"
        else:
            path_in_repo = f"crates/core/src/{module_path}.rs"
            if not os.path.isfile(f"../../../{path_in_repo}"):
                path_in_repo = f"crates/core/src/{module_path}/mod.rs"
        src_to_include = Path(f"../../../{path_in_repo}").read_text()
        module_rust = module.replace("::", " { pub mod ")
        suffix += f"\n\n#[allow(dead_code, unused_imports, unused_macros)]\nmod {module_rust} {{\n"
        suffix += f"    // This is https://github.com/fornwall/advent-of-code/tree/master/{path_in_repo} inlined to work in the Rust Playground."
        for line in iter(src_to_include.splitlines()):
            if line:
                suffix += f"\n    {line}"
            else:
                suffix += "\n"
        suffix += "\n" + "}" * (1 + module.count("::"))
        found = True

    src = re.sub(r"use super::(.*)?::", lambda match: f"use {match.group(1)}::", src)
    src = re.sub(r"use crate::(.*)?::", lambda match: f"use {match.group(1)}::", src)

    return header + "\n\n" + src + suffix


def replace_include_str(dirpath, src):
    def replace(match):
        included_file = match.group(1)
        replacement_file = os.path.join(dirpath, included_file)
        included_src = Path(replacement_file).read_text()
        included_src = included_src.replace("\\", "\\\\").replace("\n", "\\n").replace('"', '\\"')
        return f'"{included_src}"'

    return re.sub(r'include_str!\("(.*?)"\)', replace, src)

def get_gist(gist_id):
    API_TOKEN = os.environ["GIST_API_TOKEN"]
    get_response = requests.get(f"https://api.github.com/gists/{gist_id}", headers={
        "authorization": f"token {API_TOKEN}",
        "accept": "application/vnd.github.v3+json",
        "content-type": "application/json",
    })
    return get_response.json()

def create_compiler_explorer_link(year, day, src, link_id=None):
    client_state = {
        "sessions": [{
                "id": 1,
                "language": "rust",
                "source": src,
                "compilers": [{ "id": "beta", "options": "--edition=2021 -C opt-level=2" }],
        }]
    }
    response = requests.post('https://godbolt.org/api/shortener', json=client_state)
    return response.json()['url'].split('/')[-1]

def set_gist(year, day, src, gist_id=None):
    API_TOKEN = os.environ["GIST_API_TOKEN"]

    file_name = f"year{year}_day{day}.rs"
    headers = {
        "authorization": f"token {API_TOKEN}",
        "accept": "application/vnd.github.v3+json",
        "content-type": "application/json",
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
    return response.json()


with open(MAPPING_FILE_NAME, "r") as infile:
    gist_mapping = json.load(infile)

for (dirpath, dirnames, filenames) in os.walk("../../core/src/"):
    if not "year" in dirpath:
        continue
    year = int(dirpath.split("/")[-1][4:])
    for filename in filenames:
        if not filename.endswith(".rs"):
            continue
        if not filename.startswith("day"):
            continue
        if filename.endswith("renderer.rs"):
            continue

        day = int(filename[3:][:-3])
        path = os.path.join(dirpath, filename)

        print(f"{year} - {day}")

        src = Path(path).read_text()

        # Check if visualization before inlining (since inlined code
        # may contain 'feature = "visualization"':
        supports_visualization = '#[cfg(feature = "visualization")]' in src

        # Strip away use of visualization packages - they just bloat up gist
        # with unrelated code and may contain transitive imports:
        src = re.sub('^#\\[cfg\\(feature = "visualization"[^;]*;', '', src, flags=re.MULTILINE)

        src = add_header(src, year, day)
        src = replace_include_str(dirpath, src)

        # Finally format source code:
        src = subprocess.run(['rustfmt'], stdout=subprocess.PIPE, input=src, encoding='utf-8', check=True).stdout

        year_str = str(year)
        day_str = str(day)

        if year_str in gist_mapping and day_str in gist_mapping[year_str] and 'gist' in gist_mapping[year_str][day_str]:
            existing_id = gist_mapping[year_str][day_str]['gist']
            if dry_run:
                print(f"Would reuse existing id {existing_id}")
            else:
                set_gist(year, day, src, existing_id)
        else:
            if dry_run:
                print("Would create new!")
            else:
                response_json = set_gist(year, day, src)
                new_id = response_json['id']
                raw_url = list(response_json['files'].values())[0]['raw_url']
                if year_str not in gist_mapping:
                    gist_mapping[year_str] = {}
                if day_str not in gist_mapping[year_str]:
                    gist_mapping[year_str][day_str] = {}
                gist_mapping[year_str][day_str]['gist'] = new_id
                gist_mapping[year_str][day_str]['raw_url'] = raw_url

        if not 'raw_url' in gist_mapping[year_str][day_str]:
            response_json = get_gist(gist_mapping[year_str][day_str]['gist'])
            gist_mapping[year_str][day_str]['raw_url'] = list(response_json['files'].values())[0]['raw_url']

        if 'compiler_explorer' in gist_mapping[year_str][day_str]:
            existing_id = gist_mapping[year_str][day_str]['compiler_explorer']
            existing_code = requests.get(f'https://godbolt.org/z/{existing_id}/code/1').text
            if existing_code != src:
                del gist_mapping[year_str][day_str]['compiler_explorer']
        if not 'compiler_explorer' in gist_mapping[year_str][day_str]:
            gist_mapping[year_str][day_str]['compiler_explorer'] = create_compiler_explorer_link(year, day, src)

        gist_mapping[year_str][day_str]['visualization'] = supports_visualization

if not dry_run:
    with open(MAPPING_FILE_NAME, "w") as outfile:
        json.dump(gist_mapping, outfile, indent=2)
        outfile.write("\n")
