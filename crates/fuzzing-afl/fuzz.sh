#!/bin/bash
set -u

# Check that provided input does not give direct crash:
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

set -e

# Build instrumented binary:
cargo afl build

# Setup input and output folders:
INPUT=target/fuzzing-input
OUTPUT=target/fuzzing-output
rm -Rf $INPUT $OUTPUT
mkdir -p $INPUT $OUTPUT
# TODO: Do not overwrite between years
for year in core/src/year*; do
	echo $year
	#cp  ../core/src/year*/day*_input.txt $INPUT/
done
exit 1

cargo afl fuzz \
	-i $INPUT \
	-o $OUTPUT \
	../../target/debug/advent-of-code-afl-fuzzing
