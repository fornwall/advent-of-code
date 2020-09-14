use std::env;
use std::io::Read;

use advent_of_code::solve;

fn main() {
    let usage = || -> ! {
        eprintln!("Arguments: year day part");
        eprintln!("    where: day is 1-25 and part is 1-2");
        std::process::exit(1);
    };

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        usage();
    } else if let (Ok(year), Ok(day), Ok(part)) = (
        args[1].parse::<u16>(),
        args[2].parse::<u8>(),
        args[3].parse::<u8>(),
    ) {
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("Error reading input");

        match solve(year, day, part, input.as_ref()) {
            Ok(solution) => println!("{}", solution),
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    } else {
        usage();
    }
}
