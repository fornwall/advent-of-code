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
	echo YYYYYYYYYYYYYY
	xcodebuild -showsdks
	echo OOOOOOOOOOOOOO
  : ${MACOS_SDK:="macosx16.4"}
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
elif [ "$UNAME" = "Linux" ]; then
  cd ../..

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
else
  echo "Uknown uname: '$UNAME' (assuming windows)"
  rustup target add x86_64-pc-windows-msvc aarch64-pc-windows-msvc
  cargo $BUILD_COMMAND --target x86_64-pc-windows-msvc
  cargo $BUILD_COMMAND --target aarch64-pc-windows-msvc

  cp ../../target/x86_64-pc-windows-msvc/$AOC_BUILD_TYPE/advent_of_code_java.dll \
      java-src/src/main/resources/advent_of_code_java_x86_64.dll
  cp ../../target/aarch64-pc-windows-msvc/$AOC_BUILD_TYPE/advent_of_code_java.dll \
      java-src/src/main/resources/advent_of_code_java_aarch64.dll
fi
