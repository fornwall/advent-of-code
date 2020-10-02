#!/bin/bash
set -e -u

# Build instrumented binary:
cargo afl build

# Setup input and output folders:
INPUT=target/fuzzing-input
OUTPUT=target/fuzzing-output
rm -Rf $INPUT $OUTPUT
mkdir -p $INPUT $OUTPUT
# TODO: Do not overwrite between years
COUNT=0
for YEAR_PATH in ../core/src/year*; do
	YEAR=${YEAR_PATH: -4}
	for INPUT_FILE in ../core/src/year${YEAR}/day*_input.txt; do
		COUNT=$(( COUNT + 1 ))
		if [ $COUNT -lt 5 ]; then
			SEED_FILENAME=year${YEAR}_`basename $INPUT_FILE`
			cp $INPUT_FILE $INPUT/$SEED_FILENAME
		fi
	done
done

cargo afl fuzz \
	-t 50000 \
	-i $INPUT \
	-o $OUTPUT \
	../../target/debug/advent-of-code-afl-fuzzing
