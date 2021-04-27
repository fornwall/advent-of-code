#!/bin/sh
set -u

INPUT=$1

for year in {2015..2020}; do
  for day in {1..25}; do
    for part in 1 2; do
      COMMAND="tail -c +4 \"$INPUT\" | RUST_BACKTRACE=1 cargo run -q $year $day $part"
      echo $COMMAND
      #tail -c +4 $INPUT | cargo run -q $year $day $part > /dev/null
      sh -c "$COMMAND" > /dev/null
      EXIT_CODE=$?
      if [ $EXIT_CODE != 0 ] && [ $EXIT_CODE != 1 ]; then
        echo "Crash found"
        exit 1
      fi
    done
  done
done
