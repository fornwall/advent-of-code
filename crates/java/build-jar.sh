#!/bin/sh
set -e -u

cd java-src
gradle assemble
JARFILE=`gradle -q printBuiltJar`
cd ..

echo "Done - check java-src/build/libs"

cp $JARFILE java-example/
cd java-example
javac -cp $JARFILE Main.java
java -XX:+CheckJNICalls -cp $JARFILE:. Main \
    2019 1 1
    < ../../core/src/year2020/day01_input.txt
