#!/bin/sh
set -e -u

rm -Rf env
python3 -m venv env

# OSTYPE necessary in python 3.12.{0,1} due to https://github.com/python/cpython/issues/112252
export OSTYPE=linux

. env/bin/activate
pip install -r requirements.txt
python setup.py develop
PYTHONPATH=. python tests/test_solve.py

# About to test cli
OUTPUT=`echo 14 | PYTHONPATH=. ./env/bin/advent-of-code-py 2019 1 1`
if [ "${OUTPUT}" != 2 ]; then 
	echo "Incorrect output: $OUTPUT"
	exit 1
fi

