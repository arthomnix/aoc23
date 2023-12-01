use std::str::FromStr;

pub(crate) fn part1(input: String) {
    println!("{}", input.lines()
        .map(|l| {
            let c = l.chars().filter(|c| c.is_numeric())
                .collect::<Vec<char>>();
            let mut s = String::with_capacity(2);
            s.push(*c.first().unwrap());
            s.push(*c.last().unwrap());
            i32::from_str(&s).unwrap()
        }).sum::<i32>());
}

pub(crate) fn part2(input: String) {
    println!("{}", input.lines()
        .map(|l| {
            let mut c = l.chars().collect::<Vec<char>>();
            let mut c_rev = c.iter().cloned().rev().collect::<Vec<char>>();
            c.push(' ');
            c.push(' ');
            c_rev.push(' ');
            c_rev.push(' ');
            let mut s = c.iter().cloned().collect::<String>();
            let mut s_rev = c_rev.iter().cloned().collect::<String>();

            if c.len() >= 5 {
                for i in 0..c.len() - 5 {
                    let slice = &c[i..i + 5];
                    s = replace(slice, &s);
                }
                for i in 0..c_rev.len() - 5 {
                    let slice = &c_rev[i..i + 5];
                    s_rev = replace_rev(slice, &s_rev);
                }
            }

            let mut out = String::with_capacity(2);
            out.push(*s.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>().first().unwrap());
            out.push(*s_rev.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>().first().unwrap());
            i32::from_str(&out).unwrap()
        }).sum::<i32>());
}

fn replace(slice: &[char], s: &str) -> String {
    match slice {
        &['z', 'e', 'r', 'o', _] => s.replacen("zero", "0", 1),
        &['o', 'n', 'e', _, _] => s.replacen("one", "1", 1),
        &['t', 'w', 'o', _, _] => s.replacen("two", "2", 1),
        &['t', 'h', 'r', 'e', 'e'] => s.replacen("three", "3", 1),
        &['f', 'o', 'u', 'r', _] => s.replacen("four", "4", 1),
        &['f', 'i', 'v', 'e', _] => s.replacen("five", "5", 1),
        &['s', 'i', 'x', _, _] => s.replacen("six", "6", 1),
        &['s', 'e', 'v', 'e', 'n'] => s.replacen("seven", "7", 1),
        &['e', 'i', 'g', 'h', 't'] => s.replacen("eight", "8", 1),
        &['n', 'i', 'n', 'e', _] => s.replacen("nine", "9", 1),
        _ => s.to_string(),
    }
}

fn replace_rev(slice: &[char], s: &str) -> String {
    match slice {
        &['o', 'r', 'e', 'z', _] => s.replacen("orez", "0", 1),
        &['e', 'n', 'o', _, _] => s.replacen("eno", "1", 1),
        &['o', 'w', 't', _, _] => s.replacen("owt", "2", 1),
        &['e', 'e', 'r', 'h', 't'] => s.replacen("eerht", "3", 1),
        &['r', 'u', 'o', 'f', _] => s.replacen("ruof", "4", 1),
        &['e', 'v', 'i', 'f', _] => s.replacen("evif", "5", 1),
        &['x', 'i', 's', _, _] => s.replacen("xis", "6", 1),
        &['n', 'e', 'v', 'e', 's'] => s.replacen("neves", "7", 1),
        &['t', 'h', 'g', 'i', 'e'] => s.replacen("thgie", "8", 1),
        &['e', 'n', 'i', 'n', _] => s.replacen("enin", "9", 1),
        _ => s.to_string(),
    }
}