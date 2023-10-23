#[cxx::bridge(namespace = "aoc")]
mod ffi {
    #![allow(clippy::items_after_statements)]
    extern "Rust" {
        pub unsafe fn solve(year: u16, day: u8, part: u8, input: &str) -> Result<String>;
    }
}

/// Returns the answer for the specified problem and input.
///
/// Arguments:
/// year: The year of the problem, as in 2018 or 2019.
/// day: The day of the problem, from 1 to 25.
/// part: The part of the problem, either 1 or 2.
/// input: The input to the problem.
///
/// Returns:
/// The computed answer as text.
///
/// Raises:
/// `ValueError`: If the input was invalid.
fn solve(
    year: u16,
    day: u8,
    part: u8,
    input: &str,
    #[cfg(feature = "visualization")] painter: PainterRef,
) -> Result<String, String> {
    advent_of_code::solve(year, day, part, input)
}
