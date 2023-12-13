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

type CardWithAmount = (i32, i32, Vec<i32>, Vec<i32>);

pub(crate) fn part2(input: String) {
    let mut cards = input.lines().map(|line| {
        let (card, numbers) = line.split_once(": ").unwrap();
        let card = i32::from_str(&card.replace("Card ", "").trim()).unwrap();

        let (winning, have) = numbers.trim().split_once(" | ").unwrap();
        let winning: Vec<i32> = winning.split(" ").filter_map(parse_i32_if_present).collect();
        let have: Vec<i32> = have.split(" ").filter_map(parse_i32_if_present).collect();

        (1, card, winning, have)
    }).collect::<Vec<CardWithAmount>>();

    for i in 0..cards.len() {
        let (count, card, winning, have) = &cards[i];
        let mut matches = 0;
        for n in have {
            if winning.contains(n) {
                matches += 1;
            }
        }
        let count = *count;
        for j in 1..=matches {
            cards[i + j].0 += count;
        }
    }

    println!("{}", cards.into_iter().map(|(count, _, _, _)| count).sum::<i32>());
}
