use Reflection::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Reflection {
    Vertical { col: usize },
    Horizontal { row: usize },
}

fn find_reflection(grid: &Vec<Vec<bool>>, existing: Option<Reflection>) -> Option<Reflection> {
    for idx in 0..grid.len() - 1 {
        let mut reflection = true;
        let mut upper = idx + 1;
        let mut lower = idx;
        loop {
            if grid[upper] != grid[lower] {
                reflection = false;
            }

            if lower == 0 || upper == grid.len() - 1 {
                break;
            }

            lower -= 1;
            upper += 1;
        }
        if reflection {
            let r = Some(Horizontal { row: idx + 1 });
            if r != existing {
                return r;
            }
        }
    }

    for idx in 0..grid[0].len() - 1 {
        let mut reflection = true;
        let mut upper = idx + 1;
        let mut lower = idx;
        loop {
            for y in 0..grid.len() {
                if grid[y][upper] != grid[y][lower] {
                    reflection = false;
                }
            }

            if lower == 0 || upper == grid[0].len() - 1 {
                break;
            }

            lower -= 1;
            upper += 1;
        }
        if reflection {
            let r = Some(Vertical { col: idx + 1 });
            if r != existing {
                return r;
            }
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

pub(crate) fn part1(input: String) {
    let ans = parse_grids(input).iter().map(|grid| {
        let reflection = find_reflection(grid, None).unwrap();
        match reflection {
            Vertical { col: c } => c,
            Horizontal { row: r } => r * 100,
        }
    }).sum::<usize>();
    println!("{ans}");
}

pub(crate) fn part2(input: String) {
    let ans = parse_grids(input).iter().map(|grid| {
        let smudged_reflection = find_reflection(grid, None).unwrap();
        let mut real_reflection = None;
        'outer: for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let mut desmudged = grid.clone();
                desmudged[y][x] = !desmudged[y][x];
                let reflection = find_reflection(&desmudged, Some(smudged_reflection));
                if let Some(reflection) = reflection {
                    if reflection != smudged_reflection {
                        real_reflection = Some(reflection);
                        break 'outer;
                    }
                }
            }
        }
        let reflection = real_reflection.unwrap();
        match reflection {
            Vertical { col: c } => c,
            Horizontal { row: r } => r * 100,
        }
    }).sum::<usize>();
    println!("{ans}");
}
