#!/bin/sh
set -u

INPUT=$1

for year in 2017 2018 2019 2020; do
  for day in {1..25}; do
    for part in 1 2; do
      echo "tail -c +4 $INPUT | cargo run -q $year $day $part"
      tail -c +4 $INPUT | cargo run -q $year $day $part > /dev/null
      EXIT_CODE=$?
      if [ $EXIT_CODE != 0 ] && [ $EXIT_CODE != 1 ]; then
        echo "Crash found"
        exit 1
      fi
    done
  done
done
