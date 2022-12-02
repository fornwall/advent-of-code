#!/usr/bin/env python3

# ./get-input.py <year> <day>
# Uses https://pypi.org/project/advent-of-code-data/
import json
import os
import sys
import time
from pathlib import Path

from aocd.exceptions import PuzzleLockedError
from aocd.models import Puzzle, User

sessions_file = f"{Path.home()}/.advent-of-code.json"
with open(sessions_file) as f:
    SESSIONS = json.load(f)
    for session in SESSIONS:
        if 'USED' in session['description']:
            session_cookie = session['cookie']
            break

# Prevent webbrowser.open, which aocd calls, from opening a browser:
os.environ["BROWSER"] = "true"

year = int(sys.argv[1])
day = int(sys.argv[2])

user = User(session_cookie)
puzzle = Puzzle(year=year, day=day, user=user)
while True:
    try:
        input_data = puzzle.input_data
        break
    except PuzzleLockedError:
        print("Not unlocked yet..")
        time.sleep(3)

input_file = f'src/year{year}/day{day:02}_input.txt'
with open(input_file, 'w') as f:
    f.write(input_data)
print(f'DONE: {input_file}')
