% ADVENT-OF-CODE(1) Version 2022.0.37

NAME
====

**advent-of-code** — CLI tool for solving Advent of Code problems

SYNOPSIS
========

| **advent-of-code** \[_year_] \[_day_] \[_part_] < \[_input-file_]
| **advent-of-code** \[**-h**|**\--help**|**-v**|**\--version**]

DESCRIPTION
===========

Solves Advent of Code (https://adventofcode.com) problems.

The year, day and part is specified as program arguments.

The problem input should be supplied on stdin (see example).

Options
-------

-h, \--help

:   Prints brief usage information.

-v, \--version

:   Prints the current version number.

EXAMPLES
========

Solve the second part of the first day in 2022:

:   advent-of-code 2022 1 2 < path/to/input-file.txt

BUGS
====

Bugs can be reported on GitHub: https://github.com/fornwall/advent-of-code/issues

AUTHOR
======

Fredrik Fornwall <fredrik@fornwall.net>
