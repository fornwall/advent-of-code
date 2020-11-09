#!/usr/bin/env python3

import json
import os
import re
import sys
from pathlib import Path

import requests

MAPPING_FILE_NAME = "gist-mapping.json"


def add_header(src, year, day):
    link_to_file = f"https://github.com/fornwall/advent-of-code/tree/master/crates/core/src/year{year}/day{str(day).rjust(2, '0')}.rs"
    header = f"// Solution to Advent of Code {year}, day {day}"
    header += f"\n// This is the following file:"
    header += f"\n// {link_to_file}"
    header += "\n// Create a PR or open an issue against https://github.com/fornwall/advent-of-code"
    header += "\n// to suggest or discuss possible changes."

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


def replace_include_str(path, src):
    link_to_file = (
        f"https://github.com/fornwall/advent-of-code/tree/master/crates/core/{path}"
    )

    newstring = ""

    def replace(match):
        included_file = match.group(1)
        replacement_file = os.path.join(os.path.dirname(path), included_file)
        included_src = Path(replacement_file).read_text()
        included_src = included_src.replace("\\", "\\\\").replace("\n", "\\n")
        return f'"{included_src}"'

    return re.sub(r'include_str!\("(.*?)"\)', replace, src)


def set_gist(year, day, src, gist_id=None):
    API_TOKEN = os.environ["GITHUB_API_TOKEN"]

    file_name = f"year{year}_day{day}.rs"
    headers = {
        "Authorization": f"token {API_TOKEN}",
        "accept": "application/vnd.github.v3+json",
        "Content-Type": "application/json",
    }

    if gist_id:
        GIST_API = f"https://api.github.com/gists/{gist_id}"
        get_response = requests.get(GIST_API, headers=headers)
        existing_src = get_response.json()["files"][file_name]["content"]
        if existing_src == src:
            print("Unmodified")
            return gist_id
    else:
        GIST_API = "https://api.github.com/gists"

    payload = {
        "description": f"Solution for Advent of Code: Year {year}, day {day}.",
        "files": {file_name: {"content": src}},
    }

    if not gist_id:
        payload["public"] = False

    if gist_id:
        res = requests.patch(GIST_API, headers=headers, json=payload)
    else:
        res = requests.post(GIST_API, headers=headers, json=payload)

    j = json.loads(res.text)
    return j["id"]


with open(MAPPING_FILE_NAME, "r") as infile:
    gist_mapping = json.load(infile)

if "AOC_YEAR" in os.environ:
    years = [int(os.environ["AOC_YEAR"])]
else:
    years = [2018, 2019]
if "AOC_DAY" in os.environ:
    days = [int(os.environ["AOC_DAY"])]
else:
    days = range(1, 26)

for year in years:
    for day in days:
        print(f"{year} - {day}")
        path = f"../../core/src/year{year}/day{str(day).rjust(2, '0')}.rs"

        src = Path(path).read_text()
        src = replace_include_str(path, src)
        src = add_header(src, year, day)

        year_str = str(year)
        day_str = str(day)

        if year_str in gist_mapping and day_str in gist_mapping[year_str]:
            existing_id = gist_mapping[year_str][day_str]
            set_gist(year, day, src, existing_id)
        elif False:
            new_id = set_gist(year, day, src)
            if year_str not in gist_mapping:
                gist_mapping[year_str] = {}
            gist_mapping[year_str][day_str] = new_id

with open(MAPPING_FILE_NAME, 'w') as outfile:
    json.dump(gist_mapping, outfile, indent=2)
