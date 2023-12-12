use std::collections::HashMap;
use std::str::FromStr;

fn get_arrs(springs: &[Option<bool>], groups: &[i32]) -> usize {
    fn get_arrs_internal(springs: &[Option<bool>], offs: usize, idx: usize, groups: &[i32], memos: &mut HashMap<(usize, usize), usize>) -> usize {
        if let Some(&count) = memos.get(&(offs, idx)) {
            return count;
        }

        if idx == groups.len() {
            for i in offs..springs.len() {
                if springs[i].is_some_and(|b| !b) {
                    return 0;
                }
            }
            return 1;
        }

        let mut count = 0;
        let mut seen_damaged = false;
        let min = if idx == 0 { offs } else { offs + 1 };
        for i in min..springs.len() {
            if springs[i] == Some(false) {
                seen_damaged = true;
            }
            if springs[i] == Some(true) {
                if seen_damaged {
                    break;
                } else {
                    continue;
                }
            }

            let min = groups[idx + 1..].iter().sum::<i32>() as usize + groups.len() - idx;
            if springs.len() - offs < min {
                break;
            }

            let group_len = groups[idx];
            if i + group_len as usize > springs.len() {
                break;
            }


            let mut possible = true;

            for j in offs..i {
                if springs[j].is_some_and(|b| !b) {
                    possible = false;
                }
            }

            for j in i..group_len as usize + i {
                if springs[j].is_some_and(|b| b) {
                    possible = false;
                }
            }

            if possible {
                count += get_arrs_internal(springs, i + group_len as usize, idx + 1, groups, memos);
            }
        }

        memos.insert((offs, idx), count);

        count
    }


    get_arrs_internal(springs, 0, 0, groups, &mut HashMap::new())
}

fn day12(input: String, part2: bool) {
    println!("{}", input
        .lines()
        .map(|l| l.split_once(' ').map(|(springs, groups)| {
            let mut s = String::from(springs);
            let mut g = String::from(groups);
            if part2 {
                for _ in 0..4 {
                    s.push('?');
                    s.push_str(springs);

                    g.push(',');
                    g.push_str(groups);
                }
            }
            let springs = s.chars().map(|c| match c {
                '?' => None,
                '.' => Some(true),
                '#' => Some(false),
                _ => panic!("Invalid char in springs"),
            }).collect::<Vec<_>>();
            let groups = g
                .split(',')
                .map(|s| i32::from_str(s).unwrap())
                .collect::<Vec<_>>();
            get_arrs(&springs, &groups)
        }).unwrap())
        .sum::<usize>()
    );
}

pub(crate) fn part1(input: String) {
    day12(input, false);
}

pub(crate) fn part2(input: String) {
    day12(input, true);
}
