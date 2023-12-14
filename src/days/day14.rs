use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub(crate) fn part1(input: String) {
    let grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut total = 0;
    for x in 0..grid[0].len() {
        let mut rounded_y = 0;
        for y in 0..grid.len() {
            match grid[y][x] {
                'O' => {
                    total += grid.len() - rounded_y;
                    rounded_y += 1;
                },
                '#' => rounded_y = y + 1,
                _ => {},
            }
        }
    }
    println!("{total}");
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub(crate) fn part2(input: String) {
    let mut grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut prev_states = HashMap::new();
    let mut prev_counts = vec![];

    for i in 0..1000000000usize {
        // tilt north
        for x in 0..grid[0].len() {
            let mut rounded_y = 0;
            for y in 0..grid.len() {
                match grid[y][x] {
                    'O' => {
                        grid[y][x] = '.';
                        grid[rounded_y][x] = 'O';
                        rounded_y += 1;
                    },
                    '#' => rounded_y = y + 1,
                    _ => {},
                }
            }
        }

        // tilt west
        for y in 0..grid.len() {
            let mut rounded_x = 0;
            for x in 0..grid[0].len() {
                match grid[y][x] {
                    'O' => {
                        grid[y][x] = '.';
                        grid[y][rounded_x] = 'O';
                        rounded_x += 1;
                    },
                    '#' => rounded_x = x + 1,
                    _ => {},
                }
            }
        }


        // tilt south
        for x in 0..grid[0].len() {
            let mut rounded_y = grid.len() - 1;
            for y in (0..grid.len()).rev() {
                match grid[y][x] {
                    'O' => {
                        grid[y][x] = '.';
                        grid[rounded_y][x] = 'O';
                        rounded_y -= 1;
                    },
                    '#' => rounded_y = y - 1,
                    _ => {},
                }
            }
        }

        // tilt east
        for y in 0..grid.len() {
            let mut rounded_x = grid[0].len() - 1;
            for x in (0..grid[0].len()).rev() {
                match grid[y][x] {
                    'O' => {
                        grid[y][x] = '.';
                        grid[y][rounded_x] = 'O';
                        rounded_x -= 1;
                    },
                    '#' => rounded_x = x - 1,
                    _ => {},
                }
            }
        }

        let l = grid.len();
        let total = grid.iter().enumerate().map(|(y, line)| {
            line.iter().map(move |&c| if c == 'O' { l - y } else { 0 })
        }).flatten().sum::<usize>();

        let hash = hash(&grid);
        if let Some(it) = prev_states.get(&hash) {
            let llen = i - it;
            let tgt = 1000000000 - it;
            let ofs = tgt % llen;
            let n = it + ofs - 1;
            println!("{}", prev_counts[n]);
            break;
        } else {
            prev_states.insert(hash, i);
            prev_counts.push(total);
        }
    }
}
