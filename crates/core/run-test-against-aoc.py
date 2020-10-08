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

for session in SESSIONS:
    session_cookie = session["cookie"]
    session_description = session["description"]
    user = User(session_cookie)
    for year in [2018, 2019]:
        for day in range(1, 26):
            puzzle = Puzzle(year=year, day=day, user=user)
            input_data = puzzle.input_data
            for part in range(1, 3):
                print(f"# Year {year}, Day {day}, part {part} - {session_description}")
                forked_process = subprocess.run(
                    f"cargo run --release -q {year} {day} {part}",
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
