use std::collections::HashMap;
use Instruction::*;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Right,
    Left,
}

impl Instruction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'R' => Some(Right),
            'L' => Some(Left),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    right: String,
    left: String,
}

type Map = HashMap<String, Node>;

fn parse_input(input: String) -> (Vec<Instruction>, Map) {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let instructions = instructions.trim().chars().map(|c| Instruction::from_char(c).unwrap()).collect::<Vec<_>>();
    let nodes = nodes.lines().map(|line| {
        let l = line.split(|c: char| !c.is_ascii_alphanumeric()).filter(|s| !s.is_empty()).collect::<Vec<_>>();
        (l[0].to_string(), Node { right: l[2].to_string(), left: l[1].to_string()})
    }).collect::<Map>();

    (instructions, nodes)
}

pub(crate) fn part1(input: String) {
    let (instructions, nodes) = parse_input(input);

    let mut count = 0;
    let mut current_node = "AAA";
    for inst in instructions.iter().cycle() {
        current_node = match inst {
            Right => &nodes[current_node].right,
            Left => &nodes[current_node].left,
        };
        count += 1;
        if current_node == "ZZZ" {
            println!("{count}");
            break;
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a.rem_euclid(b))
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub(crate) fn part2(input: String) {
    let (instructions, nodes) = parse_input(input);

    let cycle = nodes
        .keys()
        .filter_map(|k| if k.ends_with('A') {
            Some(k.clone())
        } else {
            None
        })
        .map(|mut node| {
            let mut first_z = false;
            let mut cycle_len: u64 = 0;
            for inst in instructions.iter().cycle() {
                node = match inst {
                    Right => nodes[&node].right.clone(),
                    Left => nodes[&node].left.clone(),
                };
                if first_z {
                    cycle_len += 1;
                }
                if node.ends_with('Z') {
                    if !first_z {
                        first_z = true;
                    }
                    else {
                        break;
                    }
                }
            }
            cycle_len
        }).reduce(lcm).unwrap();

    println!("{cycle}");
}
