#!/usr/bin/env python3

import os
import subprocess
import sys

last_day = None
total_day_time = 0
days = []


new_env = os.environ.copy()
new_env['RUSTFLAGS'] = '-C target-cpu=native'

p = subprocess.Popen(['cargo', 'bench', '--bench', 'benchmark', '--', '--output-format=bencher', '2022'],
                     text=True,
                     stdout=subprocess.PIPE,
                     env=new_env)

for line in p.stdout:
    line = line.strip()
    if not line: continue

    # test 2022_01_1 ... bench:       23235 ns/iter (+/- 92)
    parts = [x for x in line.split(" ") if x]

    day = int(parts[1][5:7])
    time = int(parts[4]) / 1_000_000
    print(f"Day {day}..", file=sys.stderr)

    if day == last_day:
        total_day_time += time
    else:
        if last_day:
            days.append((last_day, total_day_time))
        total_day_time = time
        last_day = day
days.append((last_day, total_day_time))

total_time = sum(t for (d,t) in days)
for (d,t) in days:
    percentage = (t / total_time) * 100
    print(f"Day {d:>2}: {t:>5.2f} ms {percentage:>5.2f} %")
print("")
print(f"Total: {total_time:0.2f} ms")

