#!/bin/sh
set -e -u

cargo -q build
rm -Rf target
mkdir -p target
cd example/

CXX="c++ -std=c++14 -Wall -Werror"
if [ `uname` = Linux ]; then
    CXX="$CXX -Wl,-rpath=../../../target/debug/"
fi

HEADER_PATH=../../../target/cxxbridge/advent-of-code-cxx/src/
ln -f -s lib.rs.h $HEADER_PATH/advent-of-code.hpp

$CXX -I $HEADER_PATH -L ../../../target/debug/ main.cpp -ladvent_of_code -o ../target/example

../target/example > generated-output.txt
diff -u expected-output.txt generated-output.txt

if [ `uname` = Linux ]; then
    valgrind  ../target/example
fi
