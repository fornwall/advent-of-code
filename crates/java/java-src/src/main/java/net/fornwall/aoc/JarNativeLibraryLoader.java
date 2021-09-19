package net.fornwall.aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;
import java.util.Locale;

class JarNativeLibraryLoader {

    static void loadLibraryFromJar(String baseName) {
        try {
            var libraryName = determineLibraryName(baseName);
            var tmpFile = Files.createTempFile("java-jni-" + baseName, null).toFile();
            tmpFile.deleteOnExit();

            var in= Solver.class.getResourceAsStream("/" + libraryName);
            if (in == null) {
                throw new RuntimeException("No library named " + libraryName);
            }
            try (in) {
                Files.copy(in, tmpFile.toPath(), StandardCopyOption.REPLACE_EXISTING);
            }

            System.load(tmpFile.getAbsolutePath());
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

