#!/usr/bin/env -S uv run --script
# /// script
# dependencies = [
#   "aocd",
# ]
# ///

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

verbose = "AOC_VERBOSE" in os.environ
simd = "AOC_SIMD" in os.environ
if "AOC_KEEP_GOING" in os.environ:
    report_error = lambda x: print(x, file=sys.stderr)
else:
    report_error = sys.exit

if "AOC_YEAR" in os.environ:
    years_string = os.environ["AOC_YEAR"]
    if "-" in years_string:
        (years_start, years_end) = years_string.split("-")
        years = range(int(years_start), int(years_end) + 1)
    else:
        years = [int(years_string)]
else:
    years = [2015, 2016, 2017, 2018, 2019, 2020, 2021]

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

if "AOC_ACCOUNT" in os.environ:
    account = os.environ["AOC_ACCOUNT"]
else:
    account = "*"

for year in years:
    for day in days:
        cached_inputs = {}
        print(f"# Year {year}, Day {day}")
        for session in SESSIONS:
            session_cookie = session["cookie"]
            session_description = session["description"]
            if not (account == "*" or account == session_description):
                continue

            user = User(session_cookie)

            if verbose:
                print(f"# Year {year}, Day {day} - {session_description}")
            puzzle = Puzzle(year=year, day=day, user=user)
            input_data = puzzle.input_data

            if input_data in cached_inputs:
                if verbose:
                    print("Skipping - input already seen for " + cached_inputs[input_data])
                continue
            cached_inputs[input_data] = session_description

            for part in parts:
                if day == 25 and part == 2:
                    continue

                api_to_use = os.environ.get("AOC_API")
                if api_to_use:
                    fork_command = f"../../post-input {api_to_use} {year} {day} {part}"
                else:
                    perhaps_nightly = "+nightly" if simd else ""
                    perhaps_simd = "--features simd" if simd else ""
                    fork_command = f"cargo {perhaps_nightly} run {perhaps_simd} --release -q {year} {day} {part}"

                if verbose:
                    print(f"Running: {fork_command}")

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
                            output = f"failing-input-day-{year}-{day}-{part}.txt"
                            with open(output, "w") as outfile:
                                outfile.write(input_data)
                            report_error(
                                f"Incorrect! Expected={existing_answer}, got {result}. See {output}"
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

                    report_error(f"Failed running - input data written to {output}")
