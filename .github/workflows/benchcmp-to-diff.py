#!/usr/bin/env python3

import re
import sys

new_results = []
total_time = 0

line_count = 0

table_rows = []
max_name_len = 0
max_new_time_len = 0
max_old_time_len = 0


def transform_to_micro_seconds(string):
    value = float(re.sub("±.*", "", string))
    if string.endswith("ns"):
        return value / 1000
    elif string.endswith("ms"):
        return value * 1000
    elif string.endswith("µs"):
        return value
    return value * 1000 * 1000


for line in open(sys.argv[1]):
    line = line.strip()
    line_count += 1
    if line_count >= 3:
        start = "# "

        columns = list(filter(None, line.split(" ")))

        benchmark_name = columns[0]
        benchmark_new_time = columns[2]
        if len(columns) < 7:
            # Old benchmark did not exist
            benchmark_old_time = benchmark_new_time
        else:
            benchmark_old_time = columns[6]
        benchmark_new_time = transform_to_micro_seconds(benchmark_new_time)
        benchmark_old_time = transform_to_micro_seconds(benchmark_old_time)

        max_name_len = max(max_name_len, len(benchmark_name))
        max_new_time_len = max(max_new_time_len, len(str(benchmark_new_time)))
        max_old_time_len = max(max_old_time_len, len(str(benchmark_old_time)))

        total_time += benchmark_new_time
        speedup = benchmark_new_time / benchmark_old_time

        table_rows.append((benchmark_name, benchmark_new_time, benchmark_old_time))
        new_results.append((benchmark_new_time, benchmark_name))


# Output diff table:
title_change = "Change (%)"
desired_name_len = max_name_len
space_between_columns = 3
line_width = (
    max_name_len
    + max_new_time_len
    + max_old_time_len
    + len(title_change)
    + 3 * space_between_columns
)
print("```diff")
print(("@@" + "Benchmark Difference".center(line_width - 2) + "@@").replace(" ", " "))

# Table title:
title_name = "Name"
title_new = "Old (μs)"
title_old = "New (μs)"
desired_new_len = max_new_time_len + space_between_columns
desired_old_len = max_old_time_len + space_between_columns
desired_change_len = len(title_change) + space_between_columns
line = " " * (desired_name_len - len(title_name)) + title_name
line += " " * (desired_old_len - len(title_old)) + title_old
line += " " * (desired_new_len - len(title_new)) + title_new
line += " " * space_between_columns + title_change
print(("# " + line).replace(" ", " "))

for (name, new_time, old_time) in table_rows:
    percentage_change = int(100 * (new_time / old_time - 1))
    delta = 30
    if percentage_change < -delta:
        start = "+ "
    elif percentage_change > delta:
        start = "- "
    else:
        start = "  "
    percentage_change = ("+" if percentage_change > 0 else "") + str(percentage_change)

    new_time = f"{int(round(new_time)):,}"
    old_time = f"{int(round(old_time)):,}"
    desired_name_len = max_name_len
    desired_new_len = max_new_time_len + space_between_columns
    desired_old_len = max_old_time_len + space_between_columns
    line = " " * (desired_name_len - len(name)) + name
    line += " " * (desired_old_len - len(str(old_time))) + str(old_time)
    line += " " * (desired_new_len - len(str(new_time))) + str(new_time)
    line += " " * (desired_change_len - len(str(percentage_change))) + percentage_change
    print((start + line).replace(" ", " "))  # Non-breaking space for diff alignment
print("```")

new_results.sort(reverse=True)
print("")
print("Benchmark | Time (μs) | Time (%)")
print("--- | --: | --:")
for (time, name) in new_results:
    time_in_microseconds = time // 1000
    percentage_time = (100.0 * time) / total_time
    print(f"{name} | {time_in_microseconds:,} | {percentage_time:.1f}")
