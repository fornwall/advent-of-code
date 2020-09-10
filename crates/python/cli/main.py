from advent_of_code import solve
import sys


def main():
    if len(sys.argv) != 3:
        sys.exit("usage: advent-of-code-py DAY PART < INPUT")

    day = int(sys.argv[1])
    part = int(sys.argv[2])
    problem_input = sys.stdin.read()

    try:
        problem_output = solve(day, part, problem_input)
        print(problem_output)
    except BaseException:
        sys.exit("ERROR: Invalid input")
