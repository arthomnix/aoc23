use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_in(&self, c: (i64, i64), n: i64) -> (i64, i64) {
        let (x, y) = c;
        match self {
            Direction::Up => (x, y - n),
            Direction::Down => (x, y + n),
            Direction::Left => (x - n, y),
            Direction::Right => (x + n, y),
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Direction::Right),
            1 => Some(Direction::Down),
            2 => Some(Direction::Left),
            3 => Some(Direction::Up),
            _ => None,
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            "U" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

fn parse_input(input: String, part2: bool) -> Vec<(Direction, i64)> {
    input.lines().map(|line| {
        let v = line.split_whitespace().collect::<Vec<_>>();

        if !part2 {
            let d = Direction::from_str(v[0]).unwrap();
            let n = i64::from_str(v[1]).unwrap();
            (d, n)
        } else {
            let hex = v[2].chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>();
            let c = i64::from_str_radix(&hex, 16).unwrap();
            let d = Direction::from_i64(c & 0xF).unwrap();
            (d, c >> 4)
        }
    }).collect()
}

fn shoelace(points: &[(i64, i64)]) -> i64 {
    points.windows(2).map(|s| {
        let (x1, y1) = s[0];
        let (x2, y2) = s[1];
        (y1 + y2) * (x1 - x2)
    }).sum::<i64>().abs() / 2
}

fn pick(points: &[(i64, i64)], b: i64) -> i64 {
    shoelace(points) + 1 + b / 2
}

fn day18(input: String, part2: bool) {
    let v = parse_input(input, part2);
    let mut b = 0;
    let mut p = (0, 0);
    let mut points = vec![];
    for (dir, steps) in v {
        b += steps;
        p = dir.move_in(p, steps);
        points.push(p);
    }
    let a = pick(&points, b);
    println!("{a}");
}

pub(crate) fn part1(input: String) {
    day18(input, false);
}

pub(crate) fn part2(input: String) {
    day18(input, true);
}
