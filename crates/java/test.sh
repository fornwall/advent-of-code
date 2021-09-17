#!/bin/sh

cargo build
javac -d . *.java
echo 1010 | java -Djava.library.path="$PWD/../../target/debug" net.fornwall.aoc.Main 2020 1 1
