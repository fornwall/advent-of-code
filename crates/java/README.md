[![Package on Maven Central](https://img.shields.io/maven-central/v/net.fornwall/aoc)](https://search.maven.org/artifact/net.fornwall/aoc/)
[![javadoc](https://www.javadoc.io/badge/net.fornwall/aoc.svg)](https://www.javadoc.io/doc/net.fornwall/aoc)

# advent-of-code-java
Solutions to [Advent of Code](https://adventofcode.com/) implemented in Rust and exposed as a Java library using [JNI](https://en.wikipedia.org/wiki/Java_Native_Interface).

See [src/lib.rs](src/lib.rs), which uses the [jni-rs](https://github.com/jni-rs/jni-rs) Rust bindings to JNI to expose the solutions implemented in the [core](../core) crate. This is then used by [Solver.java](java-src/src/main/java/net/fornwall/aoc/Solver.java) and loaded by [JarNativeLibraryLoader](java-src/src/main/java/net/fornwall/aoc/JarNativeLibraryLoader.java).

# How to use
The Maven group ID is `net.fornwall` and its artifact ID is `aoc`.

To add a dependency using Maven:

```xml
<dependency>
    <groupId>net.fornwall</groupId>
    <artifactId>aoc</artifactId>
    <version>2019.12.462</version>
</dependency>
```

To add a dependency using Gradle:

```gradle
dependencies {
    implementation 'net.fornwall:aoc:2019.12.462'
}
```

Answers can then be computed using [Solver.solve()](https://www.javadoc.io/doc/net.fornwall/aoc/latest/net/fornwall/aoc/Solver.html#solve(int,int,int,java.lang.String)):

```java
import net.fornwall.aoc.Solver;
import java.nio.charset.StandardCharsets;

public class Main {

    public static void main(String[] args) throws Exception {
        var year = Integer.parseInt(args[0]);
        var day = Integer.parseInt(args[1]);
        var part = Integer.parseInt(args[2]);
        var input = new String(System.in.readAllBytes(), StandardCharsets.UTF_8);

        var answer = Solver.solve(year, day, part, input);
        System.out.println(answer);
    }

}
```
