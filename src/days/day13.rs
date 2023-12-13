use Reflection::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Reflection {
    Vertical { col: usize },
    Horizontal { row: usize },
}

fn find_reflection(grid: &Vec<Vec<bool>>, p2: bool) -> Option<Reflection> {
    for idx in 0..grid.len() - 1 {
        let mut differences = 0;
        let mut upper = idx + 1;
        let mut lower = idx;
        loop {
            for x in 0..grid[0].len() {
                if grid[upper][x] != grid[lower][x] {
                    differences += 1;
                }
            }

            if lower == 0 || upper == grid.len() - 1 {
                break;
            }

            lower -= 1;
            upper += 1;
        }

        if (!p2 && differences == 0) || (p2 && differences == 1) {
            return Some(Horizontal { row: idx + 1 });
        }
    }

    for idx in 0..grid[0].len() - 1 {
        let mut differences = 0;
        let mut upper = idx + 1;
        let mut lower = idx;
        loop {
            for y in 0..grid.len() {
                if grid[y][upper] != grid[y][lower] {
                    differences += 1;
                }
            }

            if lower == 0 || upper == grid[0].len() - 1 {
                break;
            }

            lower -= 1;
            upper += 1;
        }

        if (!p2 && differences == 0) || (p2 && differences == 1) {
            return Some(Vertical { col: idx + 1 });
        }
    }

    None
}

fn parse_grids(input: String) -> Vec<Vec<Vec<bool>>> {
    input
        .split("\n\n")
        .map(|grid| grid.lines().map(|line| line.chars().map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("invalid char in input!"),
        })
            .collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>())
        .collect::<Vec<Vec<Vec<_>>>>()
}

pub(crate) fn day13(input: String, part2: bool) {
    let ans = parse_grids(input).iter().map(|grid| {
        let reflection = find_reflection(grid, part2).unwrap();
        match reflection {
            Vertical { col: c } => c,
            Horizontal { row: r } => r * 100,
        }
    }).sum::<usize>();
    println!("{ans}");
}

pub(crate) fn part1(input: String) {
    day13(input, false);
}

pub(crate) fn part2(input: String) {
    day13(input, true);
}