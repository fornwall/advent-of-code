#![allow(clippy::panic, clippy::borrow_deref_ref)]

use core::fmt::Display;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::FromPyObject;
use pyo3::PyAny;

use ::advent_of_code::solve_raw;

fn try_to_string<'a, T: Display + FromPyObject<'a>>(object: &Bound<'a, PyAny>) -> String {
    if let Ok(value) = object.extract::<String>() {
        return value;
    }

    if let Ok(value) = object.extract::<T>() {
        return value.to_string();
    }

    "".to_string()
}

/// Returns the answer for the specified problem and input.
///
/// Args:
/// year (int): The year of the problem, as in 2018 or 2019.
/// day (int): The day of the problem, from 1 to 25.
/// part (int): The part of the problem, either 1 or 2.
/// input (str): The input to the problem.
///
/// Returns:
/// str: The computed answer as text.
///
/// Raises:
/// ValueError: If the input was invalid.
#[pyfunction]
#[pyo3(text_signature = "(year, day, part, input)")]
pub fn solve<'py>(
    year: &Bound<'py, PyAny>,
    day: &Bound<'py, PyAny>,
    part: &Bound<'py, PyAny>,
    input: &str,
) -> PyResult<String> {
    let year_value = try_to_string::<u16>(year);
    let day_value = try_to_string::<u8>(day);
    let part_value = try_to_string::<u8>(part);
    solve_raw(&year_value, &day_value, &part_value, input).map_err(PyValueError::new_err)
}

// This defines a python module. pyo3 will copy the rust doc comment
// below into a python docstring, and "python setup.py test" will run
// the tests in the docstring:

/// Solve Advent of Code problems.
///
/// This module provides a function to compute answers for
/// Advent of Code (https://adventofcode.com) problems.
///
/// See https://github.com/fornwall/advent-of-code for source code.
///
/// Example usage:
///
/// >>> from advent_of_code import solve
/// >>> solve(year=2019, day=1, part=1, input='14')
/// '2'
#[pymodule]
pub fn advent_of_code(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(solve))?;

    Ok(())
}
