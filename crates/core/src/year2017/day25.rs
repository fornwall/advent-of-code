use crate::input::Input;

struct Action {
    write_one: bool,
    move_direction: i8,
    next_state: u8,
}

struct State {
    if_zero_action: Action,
    if_one_action: Action,
}

struct Tape {
    slots: Vec<bool>,
}

impl Tape {
    const MIN_POSITION: i32 = -2000;
    const MAX_POSITION: i32 = 10000;

    fn new() -> Self {
        Self {
            slots: vec![false; (Self::MAX_POSITION - Self::MIN_POSITION) as usize],
        }
    }

    const fn position_to_idx(&self, position: i32) -> usize {
        (position - Self::MIN_POSITION) as usize
    }

    fn is_one_at(&self, position: i32) -> bool {
        let idx = self.position_to_idx(position);
        self.slots[idx]
    }

    fn set_slot(&mut self, position: i32, is_one: bool) {
        let idx = self.position_to_idx(position);
        self.slots[idx] = is_one;
    }

    fn diagnostic_checksum(&self) -> usize {
        self.slots.iter().filter(|&&b| b).count()
    }
}

pub fn solve(input: &Input) -> Result<usize, String> {
    let mut tape = Tape::new();
    let mut target_steps = 0;

    let mut states: Vec<State> = Vec::new();
    let on_error = || "Invalid input".to_string();

    for (count, text) in input.text.split("\n\n").enumerate() {
        if count == 0 {
            target_steps = text
                .split(' ')
                .nth(8)
                .ok_or_else(on_error)?
                .parse::<u32>()
                .map_err(|_| on_error())?;
        } else {
            let words: Vec<&str> = text.split(' ').collect();

            if words.len() < 69 {
                return Err(on_error());
            }

            let if_zero_action = Action {
                write_one: words[17] == "1.\n",
                move_direction: if words[27] == "right.\n" { 1 } else { -1 },
                next_state: words[35]
                    .bytes()
                    .next()
                    .ok_or_else(on_error)?
                    .checked_sub(b'A')
                    .ok_or_else(on_error)?,
            };
            let if_one_action = Action {
                write_one: words[50] == "1.\n",
                move_direction: if words[60] == "right.\n" { 1 } else { -1 },
                next_state: words[68]
                    .bytes()
                    .next()
                    .ok_or_else(on_error)?
                    .checked_sub(b'A')
                    .ok_or_else(on_error)?,
            };
            states.push(State {
                if_zero_action,
                if_one_action,
            });
        }
    }

    if states.is_empty() {
        return Err(on_error());
    }

    let mut current_state = 0;
    let mut current_position = 0_i32;

    if states.iter().any(|s| {
        usize::from(s.if_one_action.next_state.max(s.if_zero_action.next_state)) >= states.len()
    }) {
        return Err("Invalid input - reference to non-defined state".to_string());
    }

    for _ in 0..target_steps {
        let current_action = if tape.is_one_at(current_position) {
            &states[current_state].if_one_action
        } else {
            &states[current_state].if_zero_action
        };

        tape.set_slot(current_position, current_action.write_one);

        current_position += i32::from(current_action.move_direction);
        if !(Tape::MIN_POSITION..Tape::MAX_POSITION).contains(&current_position) {
            return Err(format!(
                "Too long tape - only allowed inside [{},{}]",
                Tape::MIN_POSITION,
                Tape::MAX_POSITION
            ));
        }
        current_state = current_action.next_state as usize;
    }

    Ok(tape.diagnostic_checksum())
}

#[test]
pub fn tests() {
    use crate::input::test_part_one;

    let example = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
    test_part_one!(example => 3);

    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => 633);
}
