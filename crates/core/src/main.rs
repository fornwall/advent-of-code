#![forbid(unsafe_code)]
use std::env;
use std::io::Read;

#[cfg(feature = "visualization")]
use advent_of_code::painter::MockPainter;
use advent_of_code::solve_raw;

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
            .map_err(|error| format!("Error reading input: {}", error))?;

        for _ in 0..repeat {
            let solution = solve_raw(
                year,
                day,
                part,
                input.as_ref(),
                #[cfg(feature = "visualization")]
                Box::new(MockPainter {}),
            )
            .unwrap_or_else(|error| format!("Error: {}", error));
            if repeat == 1 {
                println!("{}", solution);
            }
        }
    } else {
        usage();
    }
    Ok(())
}
