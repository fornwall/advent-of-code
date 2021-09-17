import net.fornwall.aoc.Solver;
import java.nio.charset.StandardCharsets;

public class Main {
    public static void main(String[] args) throws Exception {
        var year = Integer.parseInt(args[0]);
        var day = Integer.parseInt(args[1]);
        var part = Integer.parseInt(args[2]);
        var input = new String(System.in.readAllBytes(), StandardCharsets.UTF_8);

        System.out.println(Solver.solve(year, day, part, input));
    }
}
