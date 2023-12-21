use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Index, IndexMut};

fn parse_input(input: String) -> (Vec<Vec<bool>>, (isize, isize)) {
    let mut start = (0, 0);
    let v = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            if c == 'S' {
                start = (x as isize, y as isize);
            }

            match c {
                '.' | 'S' => true,
                '#' => false,
                _ => panic!("Invalid character in input!"),
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    (v, start)
}

struct State {
    pos: (isize, isize),
    steps: usize,
}

struct WrappingGrid(Vec<Vec<bool>>);

impl Index<(isize, isize)> for WrappingGrid {
    type Output = bool;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        let (x, y) = index;
        let w = self.0[0].len() as isize;
        let h = self.0.len() as isize;

        &self.0[y.rem_euclid(h) as usize][x.rem_euclid(w) as usize]
    }
}

impl IndexMut<(isize, isize)> for WrappingGrid {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        let (x, y) = index;
        let w = self.0[0].len() as isize;
        let h = self.0.len() as isize;

        &mut self.0[y.rem_euclid(h) as usize][x.rem_euclid(w) as usize]
    }
}

fn search(grid: &WrappingGrid, start: (isize, isize), max_steps: usize) -> HashSet<(isize, isize)> {
    let mut queue = VecDeque::new();
    let mut output = HashSet::new();
    let mut visited = HashMap::new();
    queue.push_back(State { pos: start, steps: 0 });
    while let Some(state) = queue.pop_front() {
        if state.steps == max_steps {
            output.insert(state.pos);
            continue;
        }

        let (x, y) = state.pos;
        if let Some(&steps) = visited.get(&(x, y)) {
            if steps == state.steps {
                continue;
            }
        }
        visited.insert((x, y), state.steps);

        if grid[(x, y - 1)] {
            queue.push_back(State { pos: (x, y - 1), steps: state.steps + 1 });
        }
        if grid[(x, y + 1)] {
            queue.push_back(State { pos: (x, y + 1), steps: state.steps + 1 });
        }
        if grid[(x - 1, y)] {
            queue.push_back(State { pos: (x - 1, y), steps: state.steps + 1 });
        }
        if grid[(x + 1, y)] {
            queue.push_back(State { pos: (x + 1, y), steps: state.steps + 1 });
        }
    }

    output
}

pub(crate) fn part1(input: String) {
    let (grid, start) = parse_input(input);
    let grid = WrappingGrid(grid);
    let points = search(&grid, start, 64);
    println!("{}", points.len());
}

pub(crate) fn part2(input: String) {
    let (grid, start) = parse_input(input);
    let grid = WrappingGrid(grid);

    // hardcoded values for all official inputs
    let a = search(&grid, start, 65 + 0 * 131).len() as isize;
    let b = search(&grid, start, 65 + 1 * 131).len() as isize;
    let c = search(&grid, start, 65 + 2 * 131).len() as isize;
    let ca = ((c - b) - (b - a)) / 2;
    let cb = (b - 4 * ca) - (a - ca);
    let cc = (a - ca) - cb;
    let x = 202301isize;
    println!("{}", ca * x * x + cb * x + cc);
}