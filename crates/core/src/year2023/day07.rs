use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_HANDS: usize = 1024;
    let mut hands = [Hand::default(); MAX_HANDS];
    let mut num_hands = 0;

    for (idx, line) in input.text.lines().enumerate() {
        hands[idx] = Hand::parse(line, input.is_part_two())?;
        num_hands += 1;
        if num_hands > MAX_HANDS {
            return Err("Too many hands".to_string());
        }
    }

    let hands = &mut hands[0..num_hands];
    hands.sort_unstable();
    Ok(hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid as usize)
        .sum::<usize>() as u64)
}

#[derive(Copy, Clone, Default, PartialOrd, PartialEq, Ord, Eq)]
struct Hand {
    // Lowest 20 bits are each card (4 bits per card, first card highest bits).
    // Bits 21-24 are the hand strength.
    cards: u32,
    // A bid is below 1024.
    bid: u16,
}

impl Hand {
    fn parse(s: &str, jokers: bool) -> Result<Self, String> {
        let (cards_str, bid) = s.split_once(' ').ok_or_else(on_error)?;
        if cards_str.len() != 5 {
            return Err("Not 5 cards".to_string());
        }

        let bid: u16 = bid.parse().map_err(|_| "Invalid bid".to_string())?;

        let mut cards = 0_u32;
        let mut bitset = 0_u16;
        let mut counts = [0; 14];
        let mut num_jokers = 0;

        for (idx, c) in cards_str.bytes().rev().enumerate() {
            let val = card_char_to_num(c, jokers)?;
            cards |= val << (idx * 4);
            if jokers && c == b'J' {
                num_jokers += 1;
            } else {
                counts[val as usize] += 1;
                bitset |= 1 << val;
            }
        }

        let hand_type = match (
            bitset.count_ones(),
            counts.iter().max().copied().unwrap_or_default() + num_jokers,
        ) {
            // Five of a kind:
            (_, 5) => 6,
            // Four of a kind
            (2, 4) => 5,
            // Full house:
            (2, 3) => 4,
            // Three of a kind:
            (3, 3) => 3,
            // Two pair:
            (3, 2) => 2,
            // One pair
            (4, 2) => 1,
            // High card
            _ => 0,
        };
        cards |= hand_type << 20;

        Ok(Self { cards, bid })
    }
}

fn card_char_to_num(card: u8, jokers: bool) -> Result<u32, String> {
    Ok(match card {
        b'2'..=b'9' => u32::from(card - b'1'),
        b'T' => 9,
        b'J' => {
            if jokers {
                0
            } else {
                10
            }
        }
        b'Q' => 11,
        b'K' => 12,
        b'A' => 13,
        _ => {
            return Err("Invalid card - must be one of A,K,Q,J,T,[2-9]".to_string());
        }
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    #[allow(clippy::unwrap_used)]
    fn assert_hand_strength(s: &str, expected: u32) {
        let hand = Hand::parse(s, true).unwrap();
        let strength = (hand.cards >> 20) & 0b1111;
        assert_eq!(strength, expected);
    }

    assert_hand_strength("32T3K 765", 1);
    assert_hand_strength("2345J 765", 1);
    assert_hand_strength("2245J 765", 3);
    assert_hand_strength("234JJ 765", 3);
    assert_hand_strength("2244J 765", 4);
    assert_hand_strength("2225J 765", 5);
    assert_hand_strength("23JJJ 765", 5);
    assert_hand_strength("224JJ 765", 5);
    assert_hand_strength("2JJJJ 765", 6);
    assert_hand_strength("22JJJ 765", 6);
    assert_hand_strength("222JJ 765", 6);
    assert_hand_strength("2222J 765", 6);
    assert_hand_strength("22222 765", 6);
    assert_hand_strength("JJJJJ 765", 6);

    let test_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    test_part_one_no_allocations!(test_input => 6440);
    test_part_two_no_allocations!(test_input => 5905);

    let real_input = include_str!("day07_input.txt");
    test_part_one_no_allocations!(real_input => 250_602_641);
    test_part_two_no_allocations!(real_input => 251_037_509);
}
