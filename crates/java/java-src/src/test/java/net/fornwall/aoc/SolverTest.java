package net.fornwall.aoc;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class SolverTest {

    @Test
    void testSolve() {
        var answer = Solver.solve(2019, 1, 1, "14");
        Assertions.assertEquals("2", answer);
    }

    @Test
    void testException() {
        Assertions.assertThrows(SolverException.class, () -> Solver.solve(2019, -1, 1, "14"));
        Assertions.assertThrows(SolverException.class, () -> Solver.solve(2019, 1, 1, "hello"));
    }

}
