#!/bin/bash
set -u

# Check that provided seed does not give direct crash:
cd ../core
cargo build
for input in src/year*/day*input.txt; do
	for year in 2018 2019; do
		for day in {1..25}; do
			for part in 1 2; do
				echo "$year $day $part < $input"
				cargo run -q $year $day $part < $input
				EXIT_CODE=$?
				if [ $EXIT_CODE != 0 ] && [ $EXIT_CODE != 1 ]; then
					echo "Crash found"
					exit 1
				fi
			done
		done
	done
done
