use crate::common::array_stack::ArrayStack;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_HANDS: usize = 1024;
    let mut hands = ArrayStack::<MAX_HANDS, Hand>::new();

    for line in input.text.lines() {
        hands.push(Hand::parse(line, input.is_part_two())?)?;
    }
    let hands = hands.slice_mut();

    hands.sort_unstable();
    Ok(hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid() as usize)
        .sum::<usize>() as u64)
}

#[derive(Copy, Clone, Default, PartialOrd, PartialEq, Ord, Eq)]
struct Hand {
    // Lowest 32 bits is the bid.
    // Then 20 bits for cards (4 bits per card, first card highest bits).
    // Then 3 bits for the hand strength.
    bits: u64,
}

impl Hand {
    fn parse(s: &str, jokers: bool) -> Result<Self, String> {
        let (cards_str, bid) = s.split_once(' ').ok_or_else(on_error)?;
        if cards_str.len() != 5 {
            return Err("Not 5 cards".to_string());
        }

        let bid: u32 = bid.parse().map_err(|_| "Invalid bid".to_string())?;

        let mut cards = 0_u32;
        let mut bitset = 0_u16;
        let mut counts = [0; 14];
        let mut num_jokers = 0;

        for c in cards_str.bytes() {
            let val = card_char_to_num(c, jokers)?;
            cards = (cards << 4) | val;
            if jokers && c == b'J' {
                num_jokers += 1;
            } else {
                counts[val as usize] += 1;
                bitset |= 1 << val;
            }
        }

        let max_of_one = counts.iter().max().copied().unwrap_or_default() + num_jokers;
        // Writing out all the possibilities shows that this works:
        let hand_strength = ((max_of_one + 4) - bitset.count_ones()).min(7);

        Ok(Self {
            bits: (u64::from(cards) << 32) | (u64::from(hand_strength) << 52) | u64::from(bid),
        })
    }

    const fn bid(self) -> u32 {
        (self.bits & u32::MAX as u64) as u32
    }
}

fn card_char_to_num(card: u8, jokers: bool) -> Result<u32, String> {
    Ok(match (card, jokers) {
        (b'2'..=b'9', _) => u32::from(card - b'1'),
        (b'T', _) => 9,
        (b'J', true) => 0,
        (b'J', false) => 10,
        (b'Q', _) => 11,
        (b'K', _) => 12,
        (b'A', _) => 13,
        _ => {
            return Err("Invalid card - must be one of A,K,Q,J,T,[2-9]".to_string());
        }
    })
}

#[test]
pub fn tests() {
    #[allow(clippy::unwrap_used)]
    fn assert_hand_strength(s: &str, expected: u64) {
        let hand = Hand::parse(s, true).unwrap();
        let strength = (hand.bits >> 52) & 0b1111;
        assert_eq!(strength, expected);
    }

    assert_hand_strength("23456 765", 0);
    assert_hand_strength("32T3K 765", 2);
    assert_hand_strength("2345J 765", 2);
    assert_hand_strength("2245J 765", 4);
    assert_hand_strength("234JJ 765", 4);
    assert_hand_strength("2244J 765", 5);
    assert_hand_strength("2225J 765", 6);
    assert_hand_strength("23JJJ 765", 6);
    assert_hand_strength("224JJ 765", 6);
    assert_hand_strength("2JJJJ 765", 7);
    assert_hand_strength("22JJJ 765", 7);
    assert_hand_strength("222JJ 765", 7);
    assert_hand_strength("2222J 765", 7);
    assert_hand_strength("22222 765", 7);
    assert_hand_strength("JJJJJ 765", 7);

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
