package net.fornwall.aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.Locale;

class JarNativeLibraryLoader {

    static void loadLibraryFromJar(String baseName) {
        try {
            var libraryName = determineLibraryName(baseName);
            var tmpDir = Files.createTempDirectory("java-jni-" + baseName).toFile();
            tmpDir.deleteOnExit();
            var nativeLibTmpFile = new File(tmpDir, libraryName);
            nativeLibTmpFile.deleteOnExit();

            var url = Solver.class.getResource("/" + libraryName);
            if (url == null) {
                throw new RuntimeException("No library named " + libraryName);
            }
            try (var in = url.openStream()) {
                Files.copy(in, nativeLibTmpFile.toPath());
            }
            System.load(nativeLibTmpFile.getAbsolutePath());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    private static String determineLibraryName(String baseName) {
        String prefix, suffix, archExtension;

        var OS = System.getProperty("os.name", "generic").toLowerCase(Locale.ENGLISH);
        var isLinux = OS.contains("nux");
        var isMac = OS.contains("mac") || OS.contains("darwin");

        if (isLinux || isMac) {
            prefix = "lib";
            suffix = isLinux ? ".so" : ".dylib";
            if (System.getProperty("os.arch", "none").equals("aarch64")) {
                archExtension = "_aarch64";
            } else {
                archExtension = "_x86_64";
            }
        } else if (OS.contains("win")) {
            prefix = "";
            archExtension = "";
            suffix = ".dll";
        } else {
            throw new RuntimeException("Unsupported operating system: '" + OS + "'");
        }

        return prefix + baseName + archExtension + suffix;
    }

}

