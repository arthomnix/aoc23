use std::collections::VecDeque;
use std::str::FromStr;

fn parse_i32_if_present(s: &str) -> Option<i32> {
    if s.is_empty() {
        None
    } else {
        Some(i32::from_str(s.trim()).unwrap())
    }
}

pub(crate) fn part1(input: String) {
    println!("{}", input.lines().map(|line| {
        let (card, numbers) = line.split_once(": ").unwrap();
        let card = i32::from_str(&card.replace("Card ", "").trim()).unwrap();

        let (winning, have) = numbers.trim().split_once(" | ").unwrap();
        let winning: Vec<i32> = winning.split(" ").filter_map(parse_i32_if_present).collect();
        let have: Vec<i32> = have.split(" ").filter_map(parse_i32_if_present).collect();

        let mut points = 0;

        for n in have {
            if winning.contains(&n) {
                if points == 0 {
                    points = 1;
                } else {
                    points <<= 1;
                }
            }
        }

        points
    }).sum::<i32>());
}

type Card = (i32, Vec<i32>, Vec<i32>);

pub(crate) fn part2(input: String) {
    let cards = input.lines().map(|line| {
        let (card, numbers) = line.split_once(": ").unwrap();
        let card = i32::from_str(&card.replace("Card ", "").trim()).unwrap();

        let (winning, have) = numbers.trim().split_once(" | ").unwrap();
        let winning: Vec<i32> = winning.split(" ").filter_map(parse_i32_if_present).collect();
        let have: Vec<i32> = have.split(" ").filter_map(parse_i32_if_present).collect();

        (card, winning, have)
    }).collect::<Vec<Card>>();

    let mut queue = VecDeque::from(cards.clone());
    let mut n = queue.len();

    while !queue.is_empty() {
        let card = queue.pop_front().unwrap();
        let mut matches = 0;
        for n in card.2 {
            if card.1.contains(&n) {
                matches += 1;
            }
        }

        let id = card.0 as usize;
        for i in id..id + matches {
            queue.push_back(cards[i].clone());
            n += 1;
        }
    }

    println!("{n}");
}
