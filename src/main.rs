#[cfg(not(target_arch = "wasm32"))]
use std::env;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;

#[cfg(not(target_arch = "wasm32"))]
use advent_of_code_rs::get_problem_set;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let usage = || -> ! {
        eprintln!("Arguments: day part");
        eprintln!("    where: day is between 1 and 25 and part is 1 or 2");
        std::process::exit(1);
    };

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
    } else if let (Ok(day @ 1..=25), Ok(part @ 1..=2)) =
        (args[1].parse::<u8>(), args[2].parse::<u8>())
    {
        let solver = get_problem_set(day, part).unwrap();
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
