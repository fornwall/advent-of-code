#!/bin/sh
set -e -u

cd java-src
gradle assemble
cd ..

echo "Done - check java-src/build/libs"

JARFILE=advent-of-code-0.1.0.jar
cp java-src/build/libs/$JARFILE java-example/
cd java-example
javac -cp $JARFILE Main.java
java -cp $JARFILE:. Main \
    2020 1 1 \
    < ../../core/src/year2020/day01_input.txt
