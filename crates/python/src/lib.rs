#![allow(clippy::panic)]
use advent_of_code::solve_raw;
use core::fmt::Display;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::FromPyObject;
use pyo3::PyAny;

fn try_to_string<'a, T: Display + FromPyObject<'a>>(object: &'a PyAny) -> String {
    if let Ok(value) = object.extract::<String>() {
        value
    } else if let Ok(value) = object.extract::<T>() {
        value.to_string()
    } else {
        "".to_string()
    }
}

/// Returns the solution for the specified given problem and input.
///
/// # Arguments
///
/// * `year` - The year of the problem, as in 2018 or 2019.
/// * `day` - The day of the problem - from 1 to 25.
/// * `part` - The part of the problem - either 1 or 2.
/// * `input` - The input to the problem.
#[pyfunction]
pub fn solve(year: &PyAny, day: &PyAny, part: &PyAny, input: &str) -> PyResult<String> {
    let year_value = try_to_string::<u16>(year);
    let day_value = try_to_string::<u8>(day);
    let part_value = try_to_string::<u8>(part);
    solve_raw(&year_value, &day_value, &part_value, input).map_err(PyValueError::new_err)
}

// This defines a python module. pyo3 will copy the rust doc comment
// below into a python docstring, and "python setup.py test" will run
// the tests in the docstring:

/// A package with solutions for Advent of Code problems
///
/// This package provides python bindings for the rust crate
/// [cpp_demangle](http://github.com/gimli-rs/cpp_demangle) by building
/// a native Python extension using [PyO3](https://github.com/pyO3/pyO3)
///
/// Basic usage:
///
/// >>> sum_as_string(1, 2)
/// '4'
#[pymodule]
fn advent_of_code(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(solve))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
