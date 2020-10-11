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
for YEAR_PATH in ../core/src/year{2018,2019}; do
	YEAR=${YEAR_PATH: -4}
	for INPUT_FILE in ../core/src/year${YEAR}/day*_input.txt; do
		COUNT=$(( COUNT + 1 ))
		if [ $COUNT -lt 5 ]; then
			SEED_FILENAME=year${YEAR}_`basename $INPUT_FILE`
			cp $INPUT_FILE $INPUT/$SEED_FILENAME
		fi
	done
done

set +e
timeout 1800s cargo afl fuzz \
	-t 50000 \
	-i $INPUT \
	-o $OUTPUT \
	../../target/debug/advent-of-code-afl-fuzzing

mkdir -p target/crashes-to-upload
rm -Rf target/fuzzing-output/crashes/README.txt
COUNT=0
for file in target/fuzzing-output/crashes/*; do
   (( COUNT +=1 ))
   mv $file target/crashes-to-upload/crash_$COUNT.txt
done
