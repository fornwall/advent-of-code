use super::assembly::{Instruction, Program, Value};
use crate::input::Input;

fn is_prime(number: i32) -> bool {
    let number_sqrt = f64::from(number).sqrt() as i32;
    (2..=number_sqrt).all(|i| number % i != 0)
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut program = Program::parse(input.text)?;
    if input.is_part_one() {
        program.run_until_recover(None);
        Ok(program.mul_count)
    } else {
        // Register a is set to 1 at start.
        //
        //  0: set b 67         b = 67
        //  1: set c b          c = b
        //  2: jnz a 2          if a == 0:     // True in part 1, false in part 2.
        //  3: jnz 1 5              GOTO 5+
        //  4: mul b 100        b *= 100       // b =   6700
        //  5: sub b -100000    b += 100000    // b = 106700
        //  6: set c b          c = b          // c = 106700
        //  7: sub c -17000     c += 17000     // c = 123700
        //
        // Initialization done, now entering main loop with (b = 106700, c = 123700):
        //
        //                      b = 106700
        //                      c = 123700
        //  8: set f 1          while (true) { f = 1
        //  9: set d 2            d = 2
        // 10: set e 2            outer_loop: { e = 2
        // 11: set g d              inner_loop: {                            // g = d
        // 12: mul g e                                                       // g *= e
        // 13: sub g b                                                       // g -= b
        // 14: jnz g 2                if b == d * e:
        // 15: set f 0                   f = 0
        // 16: sub e -1               e += 1
        // 17: set g e                                                       // g = e
        // 18: sub g b                                                       // g -= b
        // 19: jnz g -8               if b != e: continue inner_loop
        // 20: sub d -1               d += 1
        // 21: set g d                                                       // g = d - b
        // 22: sub g b                                                       // g -= b
        // 23: jnz g -13              if b != d: continue outer_loop
        // 24: jnz f 2                if f == 0:
        // 25: sub h -1                 h += 1
        // 26: set g b                                                       // g = b - 123700
        // 27: sub g c                                                       // g -= c
        // 28: jnz g 2                if b == 123700:
        // 29: jnz 1 3                  exit()
        // 30: sub b -17              b += 17
        // 31: jnz 1 -23        }}}
        //
        // Rewritten, with f renamed to false and considered a boolean, h renamed to count:
        //
        // count = 0
        // for b in 106700..=123700, step by 17:
        //     found = false
        //     for d in 2..b:
        //         for e in 2..b:
        //             if b == d * e: found = true
        //     if found: count += 1
        // return count
        //
        // So the program is counting the number of non-prime values of b.
        let start_value = {
            match program.instructions[0] {
                Instruction::Set(_, Value::Number(number)) => 100 * (number as i32) + 100_000,
                _ => {
                    return Err("Unsupported program".to_string());
                }
            }
        };
        let end_value = start_value + 17_000;
        Ok((start_value..=end_value)
            .step_by(17)
            .filter(|&n| !is_prime(n))
            .count() as u32)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => 4225);
    test_part_two!(real_input => 905);
}
