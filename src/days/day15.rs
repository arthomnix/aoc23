use std::str::FromStr;

fn hash(s: &str) -> u8 {
    let mut cval: u64 = 0;
    for c in s.chars() {
        cval += c as u64;
        cval *= 17;
    }
    cval as u8
}

pub(crate) fn part1(input: String) {
    println!("{}", input.replace('\n', "").split(',').map(|s| hash(s) as u64).sum::<u64>());

}

const EMPTY_VEC: Vec<(&str, u8)> = vec![];

pub(crate) fn part2(input: String) {
    let mut lenses: [Vec<(&str, u8)>; 256] = [EMPTY_VEC; 256];
    let input = input.replace('\n', "");
    let sequence = input.split(',').collect::<Vec<&str>>();
    for s in sequence {
        if s.ends_with('-') {
            let label = &s[..s.len() - 1];
            let hash = hash(label);
            let lbox = &mut lenses[hash as usize];
            if let Some(index) = lbox.iter().position(|(s, _)| *s == label) {
                lbox.remove(index);
            }
        } else {
            let (label, n) = s.split_once('=').unwrap();
            let hash = hash(label);
            let lbox = &mut lenses[hash as usize];
            let n = u8::from_str(n).unwrap();
            if let Some(index) = lbox.iter().position(|(s, _)| *s == label) {
                lbox[index].1 = n;
            } else {
                lbox.push((label, n));
            }
        }
    }

    let mut power = 0;
    for (nbox, lbox) in lenses.into_iter().enumerate() {
        for (slot, lens) in lbox.into_iter().enumerate() {
            power += (nbox + 1) * (slot + 1) * lens.1 as usize;
        }
    }

    println!("{power}");
}
