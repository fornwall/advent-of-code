use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u32, String> {
    const MAX_CARDS: usize = 256;

    let num_cards = input.text.lines().count();
    if num_cards > MAX_CARDS {
        return Err(format!("Too many cards: {num_cards}"));
    }
    let cards = &mut [1; MAX_CARDS][0..num_cards];

    let mut sum = 0;
    for (card_idx, card_str) in input.text.lines().enumerate() {
        let card_str = card_str.split_once(": ").ok_or_else(on_error)?.1;

        let (win_numbers, my_numbers) = card_str.split_once(" | ").ok_or_else(on_error)?;

        let mut winning = 0_u128;
        for number in win_numbers.split_ascii_whitespace() {
            let number = parse_number(number)?;
            winning |= 1 << number;
        }

        let mut this_score = 0;
        for number in my_numbers.split_ascii_whitespace() {
            let number = parse_number(number)?;
            if winning & (1 << number) != 0 {
                this_score = if input.is_part_one() && this_score != 0 {
                    this_score * 2
                } else {
                    this_score + 1
                };
            }
        }

        if input.is_part_one() {
            sum += this_score;
        } else {
            let num_copies = cards[card_idx];
            for i in card_idx..(card_idx + this_score).min(cards.len() - 1) {
                cards[i + 1] += num_copies;
            }
        }
    }
    Ok(if input.is_part_one() {
        sum as u32
    } else {
        cards.iter().sum::<u32>()
    })
}

fn parse_number(num_str: &str) -> Result<u8, String> {
    let n = num_str.parse::<u8>().map_err(|_| on_error())?;
    if n >= 128 {
        return Err(on_error());
    }
    Ok(n)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    test_part_one_no_allocations!(test_input => 13);
    test_part_two_no_allocations!(test_input => 30);

    let real_input = include_str!("day04_input.txt");
    test_part_one_no_allocations!(real_input => 17803);
    test_part_two_no_allocations!(real_input => 5_554_894);
    let real_input = include_str!("day04_input_other.txt");
    test_part_two_no_allocations!(real_input => 6_420_979);
}
