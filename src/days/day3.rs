use std::collections::{HashMap, HashSet};

fn get_adjacent_positions(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut v = vec![];

    let min_x = (x - 1) as isize;
    let min_y = (y - 1) as isize;

    for vy in min_y..=min_y + 2 {
        for vx in min_x..=min_x + 2 {
            if vx > 0 && vy > 0 && vx < (w - 1) as isize && vy < (h - 1) as isize && (vx != x as isize || vy != y as isize) {
                v.push((vx as usize, vy as usize));
            }
        }
    }

    v
}

fn process_number(sum: &mut i32, tmp_number: &mut i32, adjacent: &mut bool, gear_positions: &mut HashSet<(usize, usize)>, gears: &mut HashMap<(usize, usize), Vec<i32>>) {
    if *adjacent {
        *sum += *tmp_number;
        *adjacent = false;

        for (x, y) in gear_positions.iter() {
            if !gears.contains_key(&(*x, *y)) {
                gears.insert((*x, *y), vec![*tmp_number]);
            } else {
                gears.get_mut(&(*x, *y)).unwrap().push(*tmp_number);
            }
        }
    }
    gear_positions.clear();
    *tmp_number = 0;
}

fn day3(input: String, part2: bool) {
    let schematic = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let w = schematic[0].len();
    let h = schematic.len();

    let mut tmp_number = 0;
    let mut adjacent = false;
    let mut gear_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for y in 0..h {
        for x in 0..w {
            if schematic[y][x].is_ascii_digit() {
                tmp_number *= 10;
                tmp_number += (schematic[y][x] as i32) & 0x0f;

                for (ax, ay) in get_adjacent_positions(x, y, w, h) {
                    if schematic[ay][ax].is_ascii_punctuation() && schematic[ay][ax] != '.' {
                        adjacent = true;
                        if schematic[ay][ax] == '*' {
                            gear_positions.insert((ax, ay));
                        }
                    }
                }
            } else {
                process_number(&mut sum, &mut tmp_number, &mut adjacent, &mut gear_positions, &mut gears);
            }
        }
        process_number(&mut sum, &mut tmp_number, &mut adjacent, &mut gear_positions, &mut gears);
    }

    if part2 {
        println!("{}", gears.into_iter().filter_map(|((x, y), v)| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        }).sum::<i32>());
    } else {
        println!("{sum}");
    }
}

pub(crate) fn part1(input: String) {
    day3(input, false);
}

pub(crate) fn part2(input: String) {
    day3(input, true);
}
