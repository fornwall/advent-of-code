#!/usr/bin/env python3

import sys

new_results = []
total_time = 0

print("```diff")
first_line = True
for line in open(sys.argv[1]):
    line = line.strip()
    if first_line:
        first_line = False
        print("@@" + "Benchmark Diff".center(len(line) - 2) + "@@")
        start = "# "
    else:
        columns = line.split(" ")

        # name
        # old-benchmark.txt ns/iter
        # new-benchmark.txt ns/iter
        # diff ns/iter
        # diff %
        benchmark_name = columns[0]
        benchmark_new_time = int(columns[2].replace(',', ''))
        total_time += benchmark_new_time
        speedup = float(columns[-1])

        new_results.append((benchmark_new_time, benchmark_name))

        delta = 0.3
        if speedup < 1 - delta:
            start = "- "
        elif speedup > 1 + delta:
            start = "+ "
        else:
            start = "  "

    print((start + line).replace(' ', ' ')) # Non-breaking space for diff alignment
print("```")

new_results.sort(reverse=True)
print('')
print('Benchmark | Time (μs) | Time (%)')
print('--- | --- | ---')
for (time, name) in new_results:
    time_in_microseconds = time / 1000.
    percentage_time = (100. * time) / total_time
    print(f"{name} | {time_in_microseconds:.2f} | {percentage_time:.2f}")

