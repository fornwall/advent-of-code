#!/usr/bin/env python3

# Uses https://pypi.org/project/advent-of-code-data/
import json
import os
import subprocess
import sys
import time
from pathlib import Path

from aocd.exceptions import PuzzleUnsolvedError
from aocd.models import Puzzle, User

sessions_file = f"{Path.home()}/.advent-of-code.json"
with open(sessions_file) as f:
    SESSIONS = json.load(f)

if 'AOC_YEAR' in os.environ:
    years = [int(os.environ['AOC_YEAR'])]
else:
    years = [2018, 2019]

if 'AOC_DAY' in os.environ:
    days = [int(os.environ['AOC_DAY'])]
else:
    days = range(1, 26)

if 'AOC_PART' in os.environ:
    parts = [int(os.environ['AOC_PART'])]
else:
    parts = range(1, 3)

for session in SESSIONS:
    session_cookie = session["cookie"]
    session_description = session["description"]
    user = User(session_cookie)
    for year in years:
        for day in days:
            puzzle = Puzzle(year=year, day=day, user=user)
            input_data = puzzle.input_data
            for part in parts:
                print(f"# Year {year}, Day {day}, part {part} - {session_description}")
                fork_command = f"cargo run --release -q {year} {day} {part}"
                if "AOC_CLOUDFLARE" in os.environ:
                    fork_command = f"../../post-input-cloudflare {year} {day} {part}"
                elif "AOC_NETLIFY" in os.environ:
                    fork_command = f"../../post-input-netlify {year} {day} {part}"
                elif "AOC_FLY" in os.environ:
                    fork_command = f"../../post-input-fly {year} {day} {part}"
                try:
                    forked_process = subprocess.run(
                        fork_command,
                        capture_output=True,
                        shell=True,
                        check=True,
                        input=input_data,
                        encoding="utf-8",
                    )
                    result = forked_process.stdout.strip()
                    try:
                        existing_answer = puzzle.answer_a if part == 1 else puzzle.answer_b
                        if existing_answer != result:
                            sys.exit(f"Incorrect! Expected={puzzle.answer_a}, got {result}")
                    except Exception:
                        if part == 1:
                            puzzle.answer_a = result
                        else:
                            puzzle.answer_b = result
                except subprocess.CalledProcessError:
                    print("Failed running")
