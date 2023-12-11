use std::collections::{HashMap, HashSet};

fn get_expanded_grid(input: String, insert_rows: usize) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
    let mut h = HashMap::new();

    let grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut ex = 0;
    let mut ey = 0;
    for (oy, l) in grid.iter().enumerate() {
        let mut empty_row = true;
        for (ox, &c) in l.iter().enumerate() {
            if c == '#' {
                empty_row = false;
                h.insert((ex, ey), HashSet::from([(ex, ey)]));
            }
            let mut empty_col = true;
            for y in 0..grid.len() {
                if grid[y][ox] == '#' {
                    empty_col = false;
                }
            }
            ex += 1;
            if empty_col {
                ex += insert_rows;
            }
        }
        ex = 0;
        ey += 1;
        if empty_row {
            ey += insert_rows;
        }
    }

    h
}

fn day11(input: String, insert_rows: usize) {
    let mut galaxies = get_expanded_grid(input, insert_rows);

    let mut total_distance = 0;

    let keys = galaxies.keys().cloned().collect::<Vec<_>>();
    for (x1, y1) in &keys {
        let v1 = galaxies.get(&(*x1, *y1)).unwrap().clone();
        for (x2, y2) in &keys {
            if !v1.contains(&(*x2, *y2)) {
                total_distance += x2.abs_diff(*x1) + y2.abs_diff(*y1);
            }
            galaxies.get_mut(&(*x2, *y2)).unwrap().insert((*x1, *y1));
        }
    }

    println!("{total_distance}");
}

pub(crate) fn part1(input: String) {
    day11(input, 1);
}

pub(crate) fn part2(input: String) {
    day11(input, 999999);
}
