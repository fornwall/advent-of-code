use advent_of_code_rs::get_problem_set;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn solve(day: u8, part: u8, input: String) -> PyResult<String> {
    if let Some(solver) = get_problem_set(day, part) {
        Ok(solver(&input))
    } else {
        Ok(format!(
            "Day ({}) must be between 1 and 25 and part ({}) either 1 or 2",
            day, part
        ))
    }
}

// This defines a python module. pyo3 will copy the rust doc comment
// below into a python docstring, and "python setup.py test" will run
// the tests in the docstring:

/// A package for solving Advent of Code 2019
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
fn advent_of_code_rs_python(_py: Python, m: &PyModule) -> PyResult<()> {
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
