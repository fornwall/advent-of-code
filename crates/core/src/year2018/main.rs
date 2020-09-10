#[cfg(not(target_arch = "wasm32"))]
use std::env;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;

#[cfg(not(target_arch = "wasm32"))]
use advent_of_code_rs::get_problem_set;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let usage = || {
        println!("Arguments: day part");
        println!("    where: day is between 1 and 24 and part is 1 or 2");
        std::process::exit(1);
    };

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
    }

    let day = match args[1].parse::<u8>() {
        Ok(day_value) => day_value,
        Err(_) => {
            usage();
            0
        }
    };
    let part = match args[2].parse::<u8>() {
        Ok(part_value) => part_value,
        Err(_) => {
            usage();
            0
        }
    };

    if !(day >= 1 && day <= 24) || !(part == 1 || part == 2) {
        usage();
    }

    if let Some(solver) = get_problem_set(day, part) {
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("Error reading input");

        let solution = solver(input.as_ref());

        println!("{}", solution);
    } else {
        println!("Solution for day {} part {} not implemented yet", day, part);
    }
}
