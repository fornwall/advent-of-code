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

            var in = Solver.class.getResourceAsStream("/" + libraryName);
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

        var osName = System.getProperty("os.name", "unknown").toLowerCase(Locale.ENGLISH);
        var isLinux = osName.contains("linux");
        var isMac = osName.contains("darwin") || osName.contains("os x") || osName.contains("osx");
        var isWindows = osName.contains("windows");

        if (isLinux) {
            prefix = "lib";
            suffix = ".so";
        } else if (isMac) {
            prefix = "lib";
            suffix = ".dylib";
        } else if (isWindows) {
            prefix = "";
            suffix = ".dll";
        } else {
            throw new RuntimeException("Unsupported operating system (os.name): " + osName);
        }

        var arch = System.getProperty("os.arch", "unknown").toLowerCase(Locale.ENGLISH);
        var isArm64 = arch.equals("arm-v8") || arch.equals("arm64") || arch.equals("aarch64");
        var isX86_64 = arch.equals("x86-64") || arch.equals("x86_64") || arch.equals("amd64") || arch.equals("x64");

        if (isX86_64) {
            archExtension = "_x86_64";
        } else if (isArm64 && !isWindows) {
            archExtension = "_aarch64";
        } else {
            throw new RuntimeException("Unsupported CPU (os.arch): " + osName);
        }

        return prefix + baseName + archExtension + suffix;
    }

}

