#!/bin/sh
set -e -u

cargo -q build
cd example/

CC="cc -Wall -Werror"
LIBS="-ladvent_of_code"
if [ `uname` = Linux ]; then
  # CC="$CC -Wl,-rpath=../../../target/debug/"
  LIBS="$LIBS -lm -lpthread"
fi

$CC -I ../target/ -L ../../../target/debug/ main.c $LIBS -o ../target/example
../target/example > generated-output.txt
diff -u expected-output.txt generated-output.txt

if [ `uname` = Linux ]; then
    valgrind  ../target/example
fi
