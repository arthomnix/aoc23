use std::str::FromStr;

fn day2(input: String, part2: bool) {
    println!("{}", input.lines().map(|line| {
        let (id, cubes) = line.split_once(":").unwrap();
        let id = i32::from_str(&id.replace("Game ", "")).unwrap();

        let mut iter = cubes.trim().split(";").map(|s| {
            let turn = s.trim();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for c in turn.split(", ") {
                let (n, colour) = c.split_once(" ").unwrap();
                let n = i32::from_str(n).unwrap();
                match colour {
                    "red" => red += n,
                    "green" => green += n,
                    "blue" => blue += n,
                    _ => panic!("illegal colour {colour}"),
                }
            }

            (red, green, blue)
        });

        if part2 {
            let reds = iter.clone().map(|(r, _, _)| r).max().unwrap();
            let greens = iter.clone().map(|(_, g, _)| g).max().unwrap();
            let blues = iter.map(|(_, _, b)| b).max().unwrap();
            reds * greens * blues
        } else {
            if iter.all(|(r, g, b)| r <= 12 && g <= 13 && b <= 14) {
                id
            } else {
                0
            }
        }
    }).sum::<i32>());
}

pub(crate) fn part1(input: String) {
    day2(input, false);
}

pub(crate) fn part2(input: String) {
    day2(input, true);
}
