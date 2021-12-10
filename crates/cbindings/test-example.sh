#!/bin/sh
set -e -u

cargo -q build
cd example/

CC="cc -Wall -Werror"
# if [ `uname` = Linux ]; then
#    CC="$CC -Wl,-rpath=../../../target/debug/"
# fi
$CC -I ../target/ -L ../../../target/debug/ main.c -ladvent_of_code -o ../target/example
../target/example > generated-output.txt
diff -u expected-output.txt generated-output.txt

if [ `uname` = Linux ]; then
    valgrind  ../target/example
fi
