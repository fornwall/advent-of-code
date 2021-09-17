package net.fornwall.aoc;

import java.io.IOException;
import java.io.File;
import java.nio.file.Files;
import java.util.Locale;

/**
 * A solver of Advent of Code solutions.
 */
public class Solver {

    static {
        try {
            var libraryName = determineLibraryName("advent_of_code_java");
            var tmpDir = Files.createTempDirectory("advent-of-code-java").toFile();
            tmpDir.deleteOnExit();
            var nativeLibTmpFile = new File(tmpDir, libraryName);
            nativeLibTmpFile.deleteOnExit();

            var url = Solver.class.getResource("/" + libraryName);
            try (var in = url.openStream()) {
                Files.copy(in, nativeLibTmpFile.toPath());
            }
            System.load(nativeLibTmpFile.getAbsolutePath());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    private static String determineLibraryName(String baseName) {
        var OS = System.getProperty("os.name", "generic").toLowerCase(Locale.ENGLISH);
        if ((OS.contains("mac")) || (OS.contains("darwin"))) {
            String archExtension;
            if (System.getProperty("os.arch", "none").equals("aarch64")) {
                archExtension = "aarch64";
            } else {
                archExtension = "x86";
            }
            return "lib" + baseName + "_" + archExtension + ".dylib";
        } else if (OS.contains("win")) {
            return baseName + ".dll";
        } else if (OS.contains("nux")) {
            return "lib" + baseName + ".so";
        } else {
            throw new RuntimeException("Unrecognized operating system: '" + OS + "'");
        }
    }

    /**
     * Solve the specified problem with the given input.
     */
    public static native String solve(int year, int day, int part, String input);

}
