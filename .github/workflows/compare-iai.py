#!/usr/bin/env python3

import json
import sys

with open(sys.argv[1]) as json_file:
    old = json.load(json_file)
with open(sys.argv[2]) as json_file:
    new = json.load(json_file)

new_results = []
total_time = 0
table_rows = []
max_name_len = 0
max_new_time_len = 0
max_old_time_len = 0

for benchmark_name in old:
    old_values = old[benchmark_name]
    new_values = new[benchmark_name]

    old_time = old_values["instructions"]
    new_time = new_values["instructions"]

    total_time += new_time

    table_rows.append((benchmark_name, new_time, old_time))
    new_results.append((new_time, benchmark_name))

    max_name_len = max(max_name_len, len(benchmark_name))
    max_new_time_len = max(max_new_time_len, len(f"{int(round(new_time)):,}"))
    max_old_time_len = max(max_old_time_len, len(f"{int(round(old_time)):,}"))

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
title_new = "New (instructions)"
title_old = "Old (instructions)"
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
print("Benchmark | Instructions (count) | Instructions (%)")
print("--- | --: | --:")
for (time, name) in new_results:
    percentage_time = (100.0 * time) / total_time
    print(f"{name} | {int(round(time)):,} | {percentage_time:.1f}")
