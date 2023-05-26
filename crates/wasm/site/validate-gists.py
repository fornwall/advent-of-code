#!/usr/bin/env python3

import time
import json
import tempfile
import os
import re
import sys
import subprocess
from pathlib import Path
import urllib.request

import requests

MAPPING_FILE_NAME = "gist-mapping.json"

with tempfile.TemporaryDirectory() as tmpdirname:
    text_file = open(f"{tmpdirname}/Cargo.toml", "wt")
    text_file.write("[package]\nname = \"test\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\n[lib]\npath = \"lib.rs\"")
    text_file.close()

    gist_mapping = json.load(open(MAPPING_FILE_NAME))
    for (year, year_dict) in sorted(gist_mapping.items()):
        for (day, day_dict) in sorted(year_dict.items()):
            url = day_dict['raw_url']
            source = requests.get(url).text

            text_file = open(f"{tmpdirname}/lib.rs", "wt")
            text_file.write(source)
            text_file.close()

            completed_process = subprocess.run(["cargo", "test"], cwd=tmpdirname, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            status_marker = '✅' if completed_process.returncode == 0 else '❌';
            print(status_marker + ' ' + str(year) + '-' + str(day))
