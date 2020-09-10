use std::env;
use std::io::Read;

use advent_of_code::get_problem_set;

fn main() {
    let usage = || -> ! {
        eprintln!("Arguments: year day part");
        eprintln!("    where: year is 2018-2019, day is 1-25 and part is 1-2");
        std::process::exit(1);
    };

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        usage();
    } else if let (Ok(year @ 2018..=2019), Ok(day @ 1..=25), Ok(part @ 1..=2)) = (
        args[1].parse::<u16>(),
        args[2].parse::<u8>(),
        args[3].parse::<u8>(),
    ) {
        let solver = get_problem_set(year, day, part).unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("Error reading input");

        let solution = solver(input.as_ref());
        println!("{}", solution);
    } else {
        usage();
    }
}
