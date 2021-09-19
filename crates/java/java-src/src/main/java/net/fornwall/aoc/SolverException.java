package net.fornwall.aoc;

/**
 * Exception thrown when an Advent of Code solution can not be solved by the
 *
 * {@link net.fornwall.aoc.Solver#solve(int, int, int, String)}
 */
public class SolverException extends RuntimeException {

    SolverException(String message) {
        super(message);
    }

}
