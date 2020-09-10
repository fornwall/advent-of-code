from advent_of_code import solve
import sys


def main():
    if len(sys.argv) != 4:
        sys.exit("usage: advent-of-code-py YEAR DAY PART < INPUT")

    year = int(sys.argv[1])
    day = int(sys.argv[2])
    part = int(sys.argv[3])
    problem_input = sys.stdin.read()

    try:
        problem_output = solve(year, day, part, problem_input)
        print(problem_output)
    except BaseException:
        sys.exit("ERROR: Invalid input")
