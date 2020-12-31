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

# Prevent webbrowser.open, which aocd calls, from opening a browser:
os.environ["BROWSER"] = "true"

if "AOC_YEAR" in os.environ:
    years_string = os.environ["AOC_YEAR"]
    if "-" in years_string:
        (years_start, years_end) = years_string.split("-")
        years = range(int(years_start), int(years_end) + 1)
    else:
        years = [int(years_string)]
else:
    years = [2018, 2019]

if "AOC_DAY" in os.environ:
    days_string = os.environ["AOC_DAY"]
    if "-" in days_string:
        (days_start, days_end) = days_string.split("-")
        days = range(int(days_start), int(days_end) + 1)
    else:
        days = [int(days_string)]
else:
    days = range(1, 26)

if "AOC_PART" in os.environ:
    parts = [int(os.environ["AOC_PART"])]
else:
    parts = range(1, 3)

for session in SESSIONS:
    session_cookie = session["cookie"]
    session_description = session["description"]
    user = User(session_cookie)
    for year in years:
        for day in days:
            print(f"# Year {year}, Day {day} - {session_description}")
            puzzle = Puzzle(year=year, day=day, user=user)
            input_data = puzzle.input_data
            for part in parts:
                if day == 25 and part == 2:
                    continue
                print(f"# Year {year}, Day {day}, part {part} - {session_description}")

                api_to_use = os.environ.get("AOC_API")
                if api_to_use:
                    fork_command = f"../../post-input {api_to_use} {year} {day} {part}"
                else:
                    fork_command = f"cargo run --release -q {year} {day} {part}"

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
                        existing_answer = (
                            puzzle.answer_a if part == 1 else puzzle.answer_b
                        )
                        if existing_answer != result:
                            sys.exit(
                                f"Incorrect! Expected={puzzle.answer_a}, got {result}"
                            )
                    except Exception:
                        if part == 1:
                            puzzle.answer_a = result
                        else:
                            puzzle.answer_b = result
                except subprocess.CalledProcessError:
                    output = "failing-input.txt"
                    with open(output, "w") as outfile:
                        outfile.write(input_data)

                    sys.exit(f"Failed running - input data written to {output}")
