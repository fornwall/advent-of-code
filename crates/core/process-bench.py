#!/usr/bin/env python3

import subprocess

last_day = None
total_day_time = 0
days = []

p = subprocess.run(['cargo', 'bench', '--bench', 'benchmark', '--', '--output-format=bencher', '2022'], capture_output=True)

for line in p.stdout.decode("utf-8").splitlines():
    line = line.strip()
    if not line: continue

    # test 2022_01_1 ... bench:       23235 ns/iter (+/- 92)
    parts = [x for x in line.split(" ") if x]

    day = int(parts[1][5:7])
    time = int(parts[4]) / 1_000_000

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

