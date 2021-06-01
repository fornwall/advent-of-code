#![forbid(unsafe_code)]
use std::env;
use std::io::Read;

#[cfg(feature = "visualization")]
use advent_of_code::painter::MockPainter;
use advent_of_code::solve_raw;

fn main() -> Result<(), String> {
    let usage = || -> ! {
        eprintln!("Arguments: year day part");
        eprintln!("    where: day is 1-25 and part is 1-2");
        std::process::exit(1);
    };

    let args: Vec<String> = env::args().collect();

    if args.iter().any(|s| s == "-v" || s == "--version") {
        println!(env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if args.len() == 4 {
        let year = &args[1];
        let day = &args[2];
        let part = &args[3];
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .map_err(|error| format!("Error reading input: {}", error.to_string()))?;

        let solution = solve_raw(
            year,
            day,
            part,
            input.as_ref(),
            #[cfg(feature = "visualization")]
            Box::new(MockPainter {}),
        )
        .unwrap_or_else(|error| format!("Error: {}", error));
        println!("{}", solution);
    } else {
        usage();
    }
    Ok(())
}
