use std::env;
use std::io::Read;

use advent_of_code::solve_raw;

fn main() -> Result<(), String> {
    let usage = || -> ! {
        eprintln!("Arguments: year day part");
        eprintln!("    where: day is 1-25 and part is 1-2");
        std::process::exit(1);
    };

    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        let year = &args[1];
        let day = &args[2];
        let part = &args[3];
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .map_err(|error| format!("Error reading input: {}", error.to_string()))?;

        let solution = solve_raw(year, day, part, input.as_ref())?;
        println!("{}", solution);
    } else {
        usage();
    }
    Ok(())
}
