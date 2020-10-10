#!/bin/sh
set -u

INPUT=$1

for year in 2018 2019; do
  for day in {1..25}; do
    for part in 1 2; do
      echo cargo run -q $year $day $part < $INPUT
      cargo run -q $year $day $part < $INPUT > /dev/null
      EXIT_CODE=$?
      if [ $EXIT_CODE != 0 ] && [ $EXIT_CODE != 1 ]; then
        echo "Crash found"
        exit 1
      fi
    done
  done
done
