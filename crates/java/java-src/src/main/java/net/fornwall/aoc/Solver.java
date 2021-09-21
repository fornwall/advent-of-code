package net.fornwall.aoc;

/**
 * A solver of <a href="https://adventofcode.com">Advent of Code</a> problems.
 * <p>
 * Solutions are implemented in Rust and exposed to Java using JNI - see <a href="https://aoc.fornwall.net">https://aoc.fornwall.net</a>
 */
public final class Solver {

    static {
        JarNativeLibraryLoader.loadLibraryFromJar("advent_of_code_java");
    }

    private Solver() {
    }

    /**
     * Solve the specified problem with the given input.
     *
     * @param year  the year of the problem being solved
     * @param day   the day of the problem being solved (1-25)
     * @param part  the part of the problem being solved (1 or 2)
     * @param input the input text to the problem
     * @return the answer of the specified problem and input
     * @throws SolverException in case of an error happened and the problem could not be solved
     */
    public static native String solve(int year, int day, int part, String input) throws SolverException;

}
