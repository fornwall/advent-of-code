#!/bin/sh
set -e -u

rm -Rf java-src/src/main/resources
mkdir -p java-src/src/main/resources

: ${AOC_BUILD_TYPE:="release"}
if [ "$AOC_BUILD_TYPE" = "debug" ]; then
    BUILD_COMMAND="build"
elif [ "$AOC_BUILD_TYPE" = "release" ]; then
    BUILD_COMMAND="build --release"
else
    echo "Unsupported build type: '$AOC_BUILD_TYPE'"
    exit 1
fi

UNAME=`uname`
if [ "$UNAME" = "Darwin" ]; then
  cargo $BUILD_COMMAND
  cp ../../target/$AOC_BUILD_TYPE/libadvent_of_code_java.dylib \
      java-src/src/main/resources/libadvent_of_code_java_x86_64.dylib

  # See xcodebuild -showsdks:
  : ${MACOS_SDK:="macosx11.1"}
  SDKROOT=`xcrun -sdk $MACOS_SDK --show-sdk-path` \
    MACOSX_DEPLOYMENT_TARGET=$(xcrun -sdk $MACOS_SDK --show-sdk-platform-version) \
    cargo $BUILD_COMMAND --target=aarch64-apple-darwin
  cp ../../target/aarch64-apple-darwin/$AOC_BUILD_TYPE/libadvent_of_code_java.dylib \
      java-src/src/main/resources/libadvent_of_code_java_aarch64.dylib

  echo "# Before stripping:"
  ls -lha java-src/src/main/resources/*.dylib
  strip -r -u java-src/src/main/resources/*.dylib
  echo "# After stripping:"
  ls -lha java-src/src/main/resources/*.dylib
else
  cd ../..

  cross $BUILD_COMMAND --target x86_64-pc-windows-gnu --package advent-of-code-java
  cp target/x86_64-pc-windows-gnu/$AOC_BUILD_TYPE/advent_of_code_java.dll \
      crates/java/java-src/src/main/resources/

  cross $BUILD_COMMAND --target x86_64-unknown-linux-gnu --package advent-of-code-java
  cp target/x86_64-unknown-linux-gnu/$AOC_BUILD_TYPE/libadvent_of_code_java.so \
      crates/java/java-src/src/main/resources/libadvent_of_code_java_x86_64.so

  cross $BUILD_COMMAND --target aarch64-unknown-linux-gnu --package advent-of-code-java
  cp target/aarch64-unknown-linux-gnu/$AOC_BUILD_TYPE/libadvent_of_code_java.so \
      crates/java/java-src/src/main/resources/libadvent_of_code_java_aarch64.so

  echo "# Before stripping:"
  ls -lha crates/java/java-src/src/main/resources/*.so
  strip crates/java/java-src/src/main/resources/libadvent_of_code_java_x86_64.so
  aarch64-linux-gnu-strip crates/java/java-src/src/main/resources/libadvent_of_code_java_aarch64.so
  echo "# After stripping:"
  ls -lha crates/java/java-src/src/main/resources/*.so
fi
