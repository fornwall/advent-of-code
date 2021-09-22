[![Package on Maven Central](https://img.shields.io/maven-central/v/net.fornwall/aoc)](https://search.maven.org/artifact/net.fornwall/aoc/)
[![javadoc](https://www.javadoc.io/badge/net.fornwall/aoc.svg)](https://www.javadoc.io/doc/net.fornwall/aoc)

# advent-of-code-java
Solutions to [Advent of Code](https://adventofcode.com/) implemented in Rust and exposed to Java using JNI.

# How to use in your project
The Maven group ID is `net.fornwall` and its artifact ID is `aoc`.

To add this library as a dependency using Maven, use the following:

```xml
<dependency>
    <groupId>net.fornwall</groupId>
    <artifactId>aoc</artifactId>
    <version>2019.12.372</version>
</dependency>
```

To add a dependency using Gradle:

```gradle
dependencies {
    implementation 'net.fornwall:aoc:2019.12.372'
}
```

Solutions can then be obtained using `Solver.solve()`:

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
