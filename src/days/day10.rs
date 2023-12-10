use std::collections::HashSet;

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                return (x, y);
            }
        }
    }
    panic!("Grid does not contain starting position!");
}

fn connects_down(c: char) -> bool {
    "|7FS".contains(c)
}

fn connects_up(c: char) -> bool {
    "|LJS".contains(c)
}

fn connects_left(c: char) -> bool {
    "-J7S".contains(c)
}

fn connects_right(c: char) -> bool {
    "-LFS".contains(c)
}

fn get_adjacent_pipes(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = Vec::with_capacity(2);
    let (x, y) = pos;
    let c = grid[y][x];

    // oh god
    match c {
        'S' => {
            if connects_down(grid[y-1][x]) {
                v.push((x, y-1));
            }
            if connects_up(grid[y+1][x]) {
                v.push((x, y+1));
            }
            if connects_left(grid[y][x+1]) {
                v.push((x+1, y));
            }
            if connects_right(grid[y][x-1]) {
                v.push((x-1, y));
            }
        }
        '|' => {
            if connects_down(grid[y-1][x]) {
                v.push((x, y-1));
            }
            if connects_up(grid[y+1][x]) {
                v.push((x, y+1));
            }
        }
        '-' => {
            if connects_left(grid[y][x+1]) {
                v.push((x+1, y));
            }
            if connects_right(grid[y][x-1]) {
                v.push((x-1, y));
            }
        }
        'L' => {
            if connects_down(grid[y-1][x]) {
                v.push((x, y-1));
            }
            if connects_left(grid[y][x+1]) {
                v.push((x+1, y));
            }
        }
        'J' => {
            if connects_down(grid[y-1][x]) {
                v.push((x, y-1));
            }
            if connects_right(grid[y][x-1]) {
                v.push((x-1, y));
            }
        }
        '7' => {
            if connects_up(grid[y+1][x]) {
                v.push((x, y+1));
            }
            if connects_right(grid[y][x-1]) {
                v.push((x-1, y));
            }
        }
        'F' => {
            if connects_up(grid[y+1][x]) {
                v.push((x, y+1));
            }
            if connects_left(grid[y][x+1]) {
                v.push((x+1, y));
            }
        }
        _ => {}
    }

    v
}

pub(crate) fn part1(input: String) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_x, start_y) = find_start(&grid);
    let starts = get_adjacent_pipes(&grid, (start_x, start_y));
    let ((mut x1, mut y1), (mut x2, mut y2)) = (starts[0], starts[1]);
    let (mut px1, mut py1) = (start_x, start_y);
    let (mut px2, mut py2) = (start_x, start_y);

    let mut distance = 1;

    println!("{}", loop {
        let (nx1, ny1) = get_adjacent_pipes(&grid, (x1, y1)).into_iter().filter(|p| *p != (px1, py1)).next().unwrap();
        let (nx2, ny2) = get_adjacent_pipes(&grid, (x2, y2)).into_iter().filter(|p| *p != (px2, py2)).next().unwrap();
        (px1, py1) = (x1, y1);
        (px2, py2) = (x2, y2);
        (x1, y1) = (nx1, ny1);
        (x2, y2) = (nx2, ny2);
        distance += 1;

        if (x1, y1) == (x2, y2) {
            break distance;
        }
    });
}

pub(crate) fn part2(input: String) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (start_x, start_y) = find_start(&grid);
    let (mut x, mut y) = get_adjacent_pipes(&grid, (start_x, start_y))[0];
    let (mut px, mut py) = (start_x, start_y);

    let mut pipe_tiles: HashSet<(usize, usize)> = HashSet::new();
    pipe_tiles.insert((start_x, start_y));

    loop {
        pipe_tiles.insert((x, y));
        let (nx, ny) = get_adjacent_pipes(&grid, (x, y)).into_iter().filter(|p| *p != (px, py)).next().unwrap();
        (px, py) = (x, y);
        (x, y) = (nx, ny);
        if (x, y) == (start_x, start_y) {
            break;
        }
    }

    let mut count = 0;
    for (y, line) in grid.iter().enumerate() {
        let mut inside = false;
        let mut entry_connects_down = false;
        for (x, c) in line.iter().enumerate() {
            if pipe_tiles.contains(&(x, y)) {
                if connects_up(*c) && connects_down(*c) && connects_down(grid[y-1][x]) && connects_up(grid[y+1][x]) {
                    inside = !inside;
                } else if connects_right(*c) && (connects_up(*c) || connects_down(*c)) {
                    entry_connects_down = connects_down(*c);
                } else if connects_left(*c) && (connects_up(*c) || connects_down(*c)) {
                    if entry_connects_down != connects_down(*c) {
                        inside = !inside;
                    }
                }
                print!("{}", match *c {
                    '|' => '│',
                    '-' => '─',
                    'L' => '╰',
                    'J' => '╯',
                    'F' => '╭',
                    '7' => '╮',
                    _ => *c,
                });
            } else if inside {
                print!("█");
                count += 1;
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!("{count}");
}
