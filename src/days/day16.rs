use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum MirrorType {
    Empty,
    VerticalSplit,
    HorizontalSplit,
    RightMirror,
    LeftMirror,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum BeamDirection {
    Upwards,
    Downwards,
    Leftwards,
    Rightwards,
}

use MirrorType as M;
use BeamDirection as B;

fn push_boundschecked(x: usize, y: usize, ox: isize, oy: isize, w: usize, h: usize,
                      dir: BeamDirection, q: &mut VecDeque<(usize, usize, BeamDirection)>)
{
    let (xi, yi) = (x as isize, y as isize);
    if xi + ox < 0 || yi + oy < 0 || xi + ox >= w as isize || yi + oy >= h as isize {
        return;
    }

    q.push_back(((xi + ox) as usize, (yi + oy) as usize, dir));
}

fn parse_input(input: String) -> Vec<Vec<MirrorType>> {
    input.lines().map(|l| l.chars().map(|c| match c {
        '.'  => M::Empty,
        '|'  => M::VerticalSplit,
        '-'  => M::HorizontalSplit,
        '/'  => M::RightMirror,
        '\\' => M::LeftMirror,
        _ => panic!("Invalid character in input!"),
    }).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn get_energised_tiles(grid: &[Vec<MirrorType>], x: usize, y: usize, direction: BeamDirection) -> usize {
    let (w, h) = (grid[0].len(), grid.len());

    let mut queue: VecDeque<(usize, usize, BeamDirection)> = VecDeque::new();
    let mut set: HashSet<(usize, usize, BeamDirection)> = HashSet::new();
    queue.push_back((x, y, direction));

    while let Some((x, y, dir)) = queue.pop_front() {
        if !set.insert((x, y, dir)) {
            continue;
        }
        match dir {
            B::Rightwards => match grid[y][x] {
                M::Empty | M::HorizontalSplit => push_boundschecked(x, y, 1, 0, w, h, B::Rightwards, &mut queue),
                M::VerticalSplit => {
                    push_boundschecked(x, y, 0, 1, w, h, B::Downwards, &mut queue);
                    push_boundschecked(x, y, 0, -1, w, h, B::Upwards, &mut queue);
                },
                M::RightMirror => push_boundschecked(x, y, 0, -1, w, h, B::Upwards, &mut queue),
                M::LeftMirror => push_boundschecked(x, y, 0, 1, w, h, B::Downwards, &mut queue),
            },
            B::Leftwards => match grid[y][x] {
                M::Empty | M::HorizontalSplit => push_boundschecked(x, y, -1, 0, w, h, B::Leftwards, &mut queue),
                M::VerticalSplit => {
                    push_boundschecked(x, y, 0, 1, w, h, B::Downwards, &mut queue);
                    push_boundschecked(x, y, 0, -1, w, h, B::Upwards, &mut queue);
                },
                M::RightMirror => push_boundschecked(x, y, 0, 1, w, h, B::Downwards, &mut queue),
                M::LeftMirror => push_boundschecked(x, y, 0, -1, w, h, B::Upwards, &mut queue),
            },
            B::Upwards => match grid[y][x] {
                M::Empty | M::VerticalSplit => push_boundschecked(x, y, 0, -1, w, h, B::Upwards, &mut queue),
                M::HorizontalSplit => {
                    push_boundschecked(x, y, -1, 0, w, h, B::Leftwards, &mut queue);
                    push_boundschecked(x, y, 1, 0, w, h, B::Rightwards, &mut queue);
                },
                M::RightMirror => push_boundschecked(x, y, 1, 0, w, h, B::Rightwards, &mut queue),
                M::LeftMirror => push_boundschecked(x, y, -1, 0, w, h, B::Leftwards, &mut queue),
            },
            B::Downwards => match grid[y][x] {
                M::Empty | M::VerticalSplit => push_boundschecked(x, y, 0, 1, w, h, B::Downwards, &mut queue),
                M::HorizontalSplit => {
                    push_boundschecked(x, y, -1, 0, w, h, B::Leftwards, &mut queue);
                    push_boundschecked(x, y, 1, 0, w, h, B::Rightwards, &mut queue);
                },
                M::RightMirror => push_boundschecked(x, y, -1, 0, w, h, B::Leftwards, &mut queue),
                M::LeftMirror => push_boundschecked(x, y, 1, 0, w, h, B::Rightwards, &mut queue),
            }
        }
    }

    let s = set.into_iter().map(|(x, y, _)| (x, y)).collect::<HashSet<_>>();
    s.len()
}

pub(crate) fn part1(input: String) {
    let grid = parse_input(input);
    println!("{}", get_energised_tiles(&grid, 0, 0, B::Rightwards));
}

pub(crate) fn part2(input: String) {
    let grid = parse_input(input);
    let (w, h) = (grid[0].len(), grid.len());
    let mx = (0..w).map(|x| {
        let top = get_energised_tiles(&grid, x, 0, B::Downwards);
        let bottom = get_energised_tiles(&grid, x, h - 1, B::Upwards);
        top.max(bottom)
    }).max().unwrap();
    let my = (0..h).map(|y| {
        let left = get_energised_tiles(&grid, 0, y, B::Rightwards);
        let right = get_energised_tiles(&grid, w - 1, y, B::Leftwards);
        left.max(right)
    }).max().unwrap();
    let max = mx.max(my);
    println!("{max}");
}
