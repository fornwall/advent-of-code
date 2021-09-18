#!/bin/sh
set -e -u

rm -Rf java-src/src/main/resources
mkdir -p java-src/src/main/resources

BUILD_TYPE=debug
if [ "$BUILD_TYPE" = "debug" ]; then
    BUILD_COMMAND="build"
else
    BUILD_COMMAND="build --release"
fi

UNAME=`uname`
if [ "$UNAME" = "Darwin" ]; then
  cargo $BUILD_COMMAND
  cp ../../target/$BUILD_TYPE/libadvent_of_code_java.dylib java-src/src/main/resources/libadvent_of_code_java_x86.dylib

  # See xcodebuild -showsdks
  SDK=macosx11.1
  SDKROOT=$(xcrun -sdk $SDK --show-sdk-path) \
    MACOSX_DEPLOYMENT_TARGET=$(xcrun -sdk $SDK --show-sdk-platform-version) \
    cargo $BUILD_COMMAND --target=aarch64-apple-darwin
  cp ../../target/aarch64-apple-darwin/$BUILD_TYPE/libadvent_of_code_java.dylib java-src/src/main/resources/libadvent_of_code_java_aarch64.dylib


  (cd ../.. && cross $BUILD_COMMAND --target x86_64-pc-windows-gnu --package advent-of-code-java && cp target/x86_64-pc-windows-gnu/$BUILD_TYPE/advent_of_code_java.dll crates/java/java-src/src/main/resources/)
  (cd ../.. && cross $BUILD_COMMAND --target x86_64-unknown-linux-gnu --package advent-of-code-java && cp target/x86_64-unknown-linux-gnu/$BUILD_TYPE/libadvent_of_code_java.so crates/java/java-src/src/main/resources/)
else
  cargo build
  LIBNAME=libadvent_of_code_java.so
  cp ../../target/debug/$LIBNAME java-src/src/main/resources
fi

# cross build --target x86_64-pc-windows-gnu --package advent-of-code-java

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
