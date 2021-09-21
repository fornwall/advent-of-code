#!/bin/sh
set -e -u

rm -Rf env
python3 -m venv env
. env/bin/activate
pip install -r requirements.txt
python setup.py develop
python tests/test_solve.py

# About to test cli
OUTPUT=`echo 14 | ./env/bin/advent-of-code-py 2019 1 1`
if [ "${OUTPUT}" != 2 ]; then 
	echo "Incorrect output: $OUTPUT"
	exit 1
fi

