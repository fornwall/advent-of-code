#!/bin/bash
set -u

FILE_TO_CHECK=$1
PATH_TO_CHECK=../fuzzing-afl/$FILE_TO_CHECK

# Check that provided file does not cause crash:
cd ../core
for input in src/year*/day*input.txt; do
	for year in 2018 2019; do
		for day in {1..25}; do
			for part in 1 2; do
				echo "$year $day $part < $PATH_TO_CHECK"
				cargo run -q $year $day $part < $PATH_TO_CHECK
				EXIT_CODE=$?
				if [ $EXIT_CODE != 0 ] && [ $EXIT_CODE != 1 ]; then
					echo "Crash found"
					exit 1
				fi
			done
		done
	done
done
