#!/bin/bash
# Change file names to avoid invalid characters for github uploading.
set -e -u

CRASHES_DIR=$1
rm -Rf rm -Rf $CRASHES_DIR/README.txt

COUNT=0
for FILE in $CRASHES_DIR/*; do
   (( COUNT +=1 ))
   if [ -f "$FILE" ]; then
    mv $FILE $CRASHES_DIR/crash_$COUNT.txt
   fi
done

