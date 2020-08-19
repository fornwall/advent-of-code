import unittest
import advent_of_code_rs_python

assert advent_of_code_rs_python.solve(1, 1, "12") == "2"
assert advent_of_code_rs_python.solve(1, 1, "14") == "2"
assert advent_of_code_rs_python.solve(1, 1, "1969") == "654"
assert advent_of_code_rs_python.solve(1, 1, "100756") == "33583"
assert (
    advent_of_code_rs_python.solve(9, 1, "104,1125899906842624,99")
    == "1125899906842624"
)


class TestStringMethods(unittest.TestCase):
    def test_upper(self):
        self.assertEqual("foo".upper(), "FOO")
