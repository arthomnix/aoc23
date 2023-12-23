use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::rc::Rc;
use derivative::Derivative;

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Derivative)]
#[derivative(Debug)]
struct State {
    length: isize,
    pos: (usize, usize),
    #[derivative(Debug="ignore")]
    prev: Option<Rc<State>>,
}

fn clone_option_rc<T>(option: &Option<Rc<T>>) -> Option<Rc<T>> {
    option.as_ref().map(|rc| Rc::clone(rc))
}

fn longest_path_dijkstra(grid: &[Vec<char>], start_pos: (usize, usize), goal_pos: (usize, usize)) -> Option<State> {
    let (sx, sy) = start_pos;
    let mut dist = vec![vec![-1; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    let mut paths = BinaryHeap::new();
    queue.push_back(State { length: 0, pos: start_pos, prev: None });
    let mut sv = 0;
    while let Some(state) = queue.pop_front() {
        sv += 1;
        let State { length, pos: (x, y), prev } = &state;
        let length = *length;
        let (x, y) = (*x, *y);

        if (x, y) == goal_pos {
            paths.push(state);
            continue;
        }

        if length < dist[y][x] {
            continue;
        }

        let mut visited = HashSet::new();
        let mut p = clone_option_rc(prev);
        while let Some(s) = p {
            visited.insert(s.pos);
            p = clone_option_rc(&s.prev);
        }

        let mut adj = Vec::with_capacity(3);

        match grid[y][x] {
            '.' => {
                if x > 0 && !visited.contains(&(x - 1, y)) && grid[y][x - 1] != '#' {
                    adj.push((x - 1, y));
                }
                if x < grid[0].len() - 1 && !visited.contains(&(x + 1, y)) && grid[y][x + 1] != '#' {
                    adj.push((x + 1, y));
                }
                if y > 0 && !visited.contains(&(x, y - 1)) && grid[y - 1][x] != '#' {
                    adj.push((x, y - 1));
                }
                if y < grid.len() - 1 && !visited.contains(&(x, y + 1)) && grid[y + 1][x] != '#' {
                    adj.push((x, y + 1));
                }
            },
            'v' => if y < grid.len() - 1 && !visited.contains(&(x, y + 1)) && grid[y + 1][x] != '#' {
                adj.push((x, y + 1));
            },
            '>' => if x < grid[0].len() - 1 && !visited.contains(&(x + 1, y)) && grid[y][x + 1] != '#' {
                adj.push((x + 1, y));
            },
            '^' => if y > 0 && !visited.contains(&(x, y - 1)) && grid[y - 1][x] != '#' {
                adj.push((x, y - 1));
            },
            '<' => if x > 0 && !visited.contains(&(x - 1, y)) && grid[y][x - 1] != '#' {
                adj.push((x - 1, y));
            },
            _ => panic!("invalid input!"),
        }


        for (nx, ny) in adj {
            let next = State { length: length + 1, pos: (nx, ny), prev: Some(Rc::new(State { length, pos: (x, y), prev: clone_option_rc(prev) }))};
            if length + 1 > dist[ny][nx] {
                queue.push_back(next);
                dist[ny][nx] = length + 1;
            }
        }
    }

    paths.pop()
}

pub(crate) fn part1(input: String) {
    let grid = parse_input(input);
    let solution = longest_path_dijkstra(&grid, (1, 0), (grid[0].len() - 2, grid.len() - 1)).unwrap();
    let mut visited = HashSet::new();
    let mut ll = Some(Rc::new(solution.clone()));
    while let Some(state) = ll {
        visited.insert(state.pos);
        ll = clone_option_rc(&state.prev);
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
    println!();

    println!("{}", solution.length);
}


fn parse_input_part2(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().map(|c| match c {
        '#' => '#',
        _ => '.',
    }).collect()).collect()
}

fn is_intersection(grid: &[Vec<char>], point: (usize, usize)) -> bool {
    let mut v = Vec::with_capacity(4);
    let (x, y) = point;
    if x > 0 {
        v.push(grid[y][x - 1]);
    }
    if x < grid[0].len() - 1 {
        v.push(grid[y][x + 1]);
    }
    if y > 0 {
        v.push(grid[y - 1][x]);
    }
    if y < grid.len() - 1 {
        v.push(grid[y + 1][x]);
    }

    v.into_iter().filter(|&c| c != '#').count() > 2
}

fn adjacent(grid: &[Vec<char>], pos: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let mut adj = Vec::with_capacity(4);
    if x > 0 && grid[y][x - 1] == '.' {
        adj.push((x - 1, y));
    }
    if x < grid[0].len() - 1 && grid[y][x + 1] == '.' {
        adj.push((x + 1, y));
    }
    if y > 0 && grid[y - 1][x] == '.' {
        adj.push((x, y - 1));
    }
    if y < grid.len() - 1 && grid[y + 1][x] == '.' {
        adj.push((x, y + 1));
    }

    adj
}

fn next_pos(grid: &[Vec<char>], pos: (usize, usize), prev: (usize, usize)) -> (usize, usize) {
    adjacent(grid, pos).into_iter().filter(|&p| p != prev).next().unwrap()
}

fn generate_graph(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> HashMap<(usize, usize), HashMap<(usize, usize), usize>> {
    let mut poi = HashSet::from([start, end]);
    for (y, l) in grid.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == '.' && is_intersection(&grid, (x, y)) {
                poi.insert((x, y));
            }
        }
    }

    let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();

    for &(x, y) in &poi {
        for mut pos in adjacent(&grid, (x, y)) {
            let mut prev = (x, y);
            let mut weight = 0;
            while !poi.contains(&pos) {
                let tmp = pos;
                pos = next_pos(&grid, pos, prev);
                prev = tmp;
                weight += 1;
            }

            if let Some(m) = graph.get_mut(&(x, y)) {
                m.insert(pos, weight + 1);
            } else {
                let mut m = HashMap::new();
                m.insert(pos, weight + 1);
                graph.insert((x, y), m);
            }
        }
    }

    graph
}

fn search_part2(graph: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>, seen: &HashSet<(usize, usize)>, start: (usize, usize), end: (usize, usize)) -> isize {
    if start == end {
        return 0;
    }

    let mut s = seen.clone();
    s.insert(start);

    let mut d = isize::MIN;
    for adj in graph.get(&start).unwrap().keys() {
        if !s.contains(adj) {
            let n = search_part2(graph, &s, *adj, end);
            d = d.max(n + *graph.get(&start).unwrap().get(adj).unwrap() as isize);
        }
    }

    d
}

pub(crate) fn part2(input: String) {
    let grid = parse_input_part2(input);
    let start = (1, 0);
    let end = (grid[0].len() - 2, grid.len() - 1);
    let graph = generate_graph(grid, start, end);
    println!("{}", search_part2(&graph, &HashSet::new(), start, end));
}