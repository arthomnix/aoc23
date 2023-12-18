use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;
use colored::Colorize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn backwards(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        })
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    pos: (usize, usize),
    cost: usize,
    direction: Direction,
    straight_line: u8,
    prev: Option<Rc<State>>,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) {}: {} (sl: {})", self.pos.0, self.pos.1, self.direction, self.cost, self.straight_line)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn can_move(current_direction: Direction, new_direction: Direction, straight_line: u8, ultra: bool) -> bool {
    if new_direction == current_direction.backwards() {
        false
    } else if current_direction != new_direction {
        if ultra {
            straight_line >= 4
        } else {
            true
        }
    } else if ultra {
        straight_line < 10
    } else {
        straight_line < 3
    }
}

fn straight_line_restricted_dijkstra(start: (usize, usize), goal: (usize, usize), grid: &[Vec<usize>], ultra: bool) -> Option<(usize, HashMap<(usize, usize), Direction>)> {
    let (start_x, start_y) = start;
    let (goal_x, goal_y) = goal;
    let mut dist = vec![vec![[[usize::MAX; 11]; 4]; grid[0].len()]; grid.len()];
    let mut prio_queue = BinaryHeap::new();
    for n in 0..4 {
        dist[start_y][start_x][n][0] = 0;
    }

    for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
        prio_queue.push(State { pos: start, cost: 0 , direction: dir, straight_line: 0 , prev: None });
    }

    while let Some(State { pos: (x, y), cost, direction, straight_line, prev }) = prio_queue.pop() {
        let dir_idx: usize = direction.into();
        let state = State {
            pos: (x, y),
            cost,
            direction,
            straight_line,
            prev: prev.as_ref().map(|p| Rc::clone(p)),
        };

        if (x, y) == goal && (!ultra || straight_line >= 4) {
            let mut m = HashMap::new();
            let mut p = prev;
            while let Some(s) = p {
                m.insert(s.pos, s.direction);
                p = s.prev.as_ref().map(|p| Rc::clone(p));
            }
            m.insert((x, y), direction);

            return Some((cost, m));
        }

        if cost > dist[y][x][dir_idx][straight_line as usize] {
            continue;
        }

        let mut adj = Vec::with_capacity(3);

        let pprev = Rc::new(state);

        if can_move(direction, Direction::Down, straight_line, ultra) && y < grid.len() - 1 {
            let sl = if direction == Direction::Down {
                straight_line + 1
            } else {
                1
            };
            adj.push(State {
                pos: (x, y + 1),
                cost: cost + grid[y + 1][x],
                direction: Direction::Down,
                straight_line: sl,
                prev: Some(Rc::clone(&pprev)),
            });
        }
        if can_move(direction, Direction::Up, straight_line, ultra) && y > 0 {
            let sl = if direction == Direction::Up {
                straight_line + 1
            } else {
                1
            };
            adj.push(State {
                pos: (x, y - 1),
                cost: cost + grid[y - 1][x],
                direction: Direction::Up,
                straight_line: sl,
                prev: Some(Rc::clone(&pprev)),
            });
        }
        if can_move(direction, Direction::Right, straight_line, ultra) && x < grid[0].len() - 1 {
            let sl = if direction == Direction::Right {
                straight_line + 1
            } else {
                1
            };
            adj.push(State {
                pos: (x + 1, y),
                cost: cost + grid[y][x + 1],
                direction: Direction::Right,
                straight_line: sl,
                prev: Some(Rc::clone(&pprev)),
            })
        }
        if can_move(direction, Direction::Left, straight_line, ultra) && x > 0 {
            let sl = if direction == Direction::Left {
                straight_line + 1
            } else {
                1
            };
            adj.push(State {
                pos: (x - 1, y),
                cost: cost + grid[y][x - 1],
                direction: Direction::Left,
                straight_line: sl,
                prev: Some(pprev),
            })
        }

        for state in adj {
            let (x, y) = state.pos;
            let dir: usize = state.direction.into();
            let sl = state.straight_line as usize;
            let cost = state.cost;
            if cost < dist[y][x][dir][sl] {
                prio_queue.push(state);
                dist[y][x][dir][sl] = cost;
            }
        }
    }

    None
}

fn day17(input: String, part2: bool) {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| usize::from_str(&c.to_string()).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (path_length, path) = straight_line_restricted_dijkstra((0, 0), (grid[0].len() - 1, grid.len() - 1), &grid, part2).unwrap();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if x == 0 && y == 0 {
                print!("{}", "•".bright_white());
            } else if let Some(dir) = path.get(&(x, y)) {
                print!("{}", dir.to_string().bright_white());
            } else {
                let n = grid[y][x] as u8;
                print!("{}", n.to_string().truecolor(255 - n * 28, 0, n * 28));
            }
        }
        println!();
    }

    println!("\n{path_length}");
}

pub(crate) fn part1(input: String) {
    day17(input, false);
}

pub(crate) fn part2(input: String) {
    day17(input, true);
}
