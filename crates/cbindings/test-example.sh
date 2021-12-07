#!/bin/sh
set -e -u

cargo build
cd example/
cc -I ../target/ -L ../../../target/debug/ -ladvent_of_code main.c -o ../target/example
../target/example
