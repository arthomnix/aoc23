mod days;

use std::env::args;
use std::io::Read;
use std::process::exit;
use std::str::FromStr;
use days::*;

static DAYS: [[fn(String); 2]; 25] = [
    [day1::part1, day1::part2],
    [day2::part1, day2::part2],
    [day3::part1, day3::part2],
    [day4::part1, day4::part2],
    [day5::part1, day5::part2],
    [day6::part1, day6::part2],
    [day7::part1, day7::part2],
    [day8::part1, day8::part2],
    [day9::part1, day9::part2],
    [day10::part1, day10::part2],
    [day11::part1, day11::part2],
    [day12::part1, day12::part2],
    [day13::part1, day13::part2],
    [day14::part1, day14::part2],
    [day15::part1, day15::part2],
    [day16::part1, day16::part2],
    [day17::part1, day17::part2],
    [day18::part1, day18::part2],
    [day19::part1, day19::part2],
    [day20::part1, day20::part2],
    [day21::part1, day21::part2],
    [day22::part1, day22::part2],
    [day23::part1, day23::part2],
    [day24::part1, day24::part2],
    [day25::part1, day25::part2],
];

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        print_usage();
    }

    let (day_s, part_s) = args[1].split_once(':').unwrap_or_else(|| print_usage());

    let day = i32::from_str(day_s).unwrap_or_else(|_| print_usage());
    let part = i32::from_str(part_s).unwrap_or_else(|_| print_usage());

    if day < 1 || day > 25 || part < 1 || part > 2 {
        eprintln!("day must be between 1-25 and part must be 1 or 2");
        print_usage();
    }

    let mut text: String;
    if args.len() > 2 && args[2] == "real" {
        let mut aoc = libaoc::AocClient::new_from_env();
        text = aoc.get_input(2022, day).unwrap_or_else(|_| {
            eprintln!("failed to retrieve input text");
            exit(2);
        });
    } else {
        println!("Enter your puzzle input, ending with Ctrl-D (EOF): (use 'aoc23 <day>:<part> real' to automatically download your real data)");
        text = String::new();
        std::io::stdin().read_to_string(&mut text).expect("Failed to read input from stdin!");
        println!("\n");
    }

    DAYS[day as usize - 1][part as usize - 1](text);
}

fn print_usage() -> ! {
    eprintln!("Usage: aoc23 <day>:<part> [real]");
    exit(1);
}