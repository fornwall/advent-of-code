#![forbid(unsafe_code)]

use std::env;
use std::io::Read;

use advent_of_code::solve_raw;

#[allow(clippy::print_stdout)]
#[allow(clippy::print_stderr)]
fn main() -> Result<(), String> {
    let usage = || -> ! {
        eprintln!("usage: advent-of-code [year] [day] [part] < [input-file]");
        std::process::exit(1);
    };

    let args: Vec<String> = env::args().collect();

    if args.iter().any(|s| s == "-v" || s == "--version") {
        println!(env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let repeat = if let Ok(value) = env::var("AOC_REPEAT") {
        value
            .parse::<usize>()
            .map_err(|_| "Unable to parse AOC_REPEAT")?
    } else {
        1
    };

    if args.len() == 4 {
        let year = &args[1];
        let day = &args[2];
        let part = &args[3];
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .map_err(|error| format!("Error reading input: {error}"))?;

        for _ in 0..repeat {
            match solve_raw(year, day, part, input.as_ref()) {
                Ok(result) => {
                    if repeat == 1 {
                        #[cfg(feature = "visualization")]
                        println!("impl Default for Visualization(visualization)");
                        #[cfg(not(feature = "visualization"))]
                        println!("{result}");
                    }
                }
                Err(error) => {
                    eprintln!("Error: {error}");
                    std::process::exit(1);
                }
            }
        }
    } else {
        usage();
    }
    Ok(())
}
