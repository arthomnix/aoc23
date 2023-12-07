use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use HandType::*;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_hand(hand: &str, part2: bool) -> Self {
        assert_eq!(hand.len(), 5);
        let mut cards: HashMap<char, i32> = HashMap::new();
        for card in hand.chars() {
            if cards.contains_key(&card) {
                *cards.get_mut(&card).unwrap() += 1;
            } else {
                cards.insert(card, 1);
            }
        }

        if part2 {
            if hand == "JJJJJ" {
                return FiveKind;
            }

            let max_card = cards
                .iter()
                .filter(|(c, _)| **c != 'J')
                .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
                .map(|(k, v)| (*k, *v))
                .unwrap();
            if let Some(jokers) = cards.get(&'J').map(|v| *v) {
                cards.remove(&'J');
                *cards.get_mut(&max_card.0).unwrap() += jokers;
            }
        }

        match cards.len() {
            1 => FiveKind,
            2 => {
                for (_, v) in cards.iter() {
                    if *v == 4 {
                        return FourKind;
                    }
                }
                FullHouse
            }
            3 => {
                for (_, v) in cards.iter() {
                    if *v == 3 {
                        return ThreeKind;
                    }
                }
                TwoPair
            }
            4 => OnePair,
            5 => HighCard,
            _ => unreachable!(),
        }
    }
}

fn compare_cards(c1: char, c2: char, joker: u32) -> Ordering {
    fn card_to_u32(c: char, joker: u32) -> u32 {
        match c {
            '0'..='9' => c.to_digit(10).unwrap(),
            'T' => 10,
            'J' => joker,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        }
    }

    card_to_u32(c1, joker).cmp(&card_to_u32(c2, joker))
}

fn day7(input: String, part2: bool) {
    let mut hands = input
        .lines()
        .map(|l| l
            .split_once(" ")
            .map(|(r, l)| (r, i32::from_str(l).unwrap()))
            .unwrap()
        )
        .collect::<Vec<(&str, i32)>>();

    let joker_val = if part2 {
        1
    } else {
        11
    };

    hands.sort_by(|(hand1, _), (hand2, _)| {
        let type1 = HandType::from_hand(*hand1, part2);
        let type2 = HandType::from_hand(*hand2, part2);

        let h1_chars = hand1.chars().collect::<Vec<_>>();
        let h2_chars = hand2.chars().collect::<Vec<_>>();

        let cmp = type2.cmp(&type1);
        if cmp == Ordering::Equal {
            for i in 0..5 {
                let cmp = compare_cards(h1_chars[i], h2_chars[i], joker_val);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            cmp
        } else {
            cmp
        }
    });

    println!("{}", hands
        .into_iter()
        .enumerate()
        .map(|(i, (c, b))| (i as i32 + 1) * b)
        .sum::<i32>());
}

pub(crate) fn part1(input: String) {
    day7(input, false);
}

pub(crate) fn part2(input: String) {
    day7(input, true);
}
