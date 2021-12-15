import sys

from advent_of_code import solve


def main():
    if len(sys.argv) != 4:
        sys.exit("usage: advent-of-code-py YEAR DAY PART < INPUT")

    year = sys.argv[1]
    day = sys.argv[2]
    part = sys.argv[3]
    problem_input = sys.stdin.read()

    try:
        problem_output = solve(year, day, part, problem_input)
        print(problem_output)
    except Exception as error:
        sys.exit("{0}".format(error))
