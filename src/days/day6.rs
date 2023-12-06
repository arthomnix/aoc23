use std::str::FromStr;

pub(crate) fn part1(input: String) {
    let lines: Vec<&str> = input.trim().lines().collect();
    let times: Vec<i32> = lines[0].split_whitespace().filter_map(|s| i32::from_str(s).ok()).collect();
    let distances: Vec<i32> = lines[1].split_whitespace().filter_map(|s| i32::from_str(s).ok()).collect();

    let mut total = 1;

    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let mut ways = 0;
        for t in 1..time {
            let d = t * (time - t);
            if d > distance {
                ways += 1;
            }
        }
        total *= ways;
    }

    println!("{total}");
}

fn binsearch(time: i64, distance: i64, from_t: i64, to_t: i64, lb: bool) -> i64 {
    if to_t - from_t <= 1 {
        return if lb {
            to_t
        } else {
            from_t
        };
    }

    let guess = (from_t + to_t) / 2;
    let d = guess * (time - guess);

    if d > distance {
        if lb {
            binsearch(time, distance, from_t, guess, lb)
        } else {
            binsearch(time, distance, guess, to_t, lb)
        }
    } else {
        if lb {
            binsearch(time, distance, guess, to_t, lb)
        } else {
            binsearch(time, distance, from_t, guess, lb)
        }
    }
}

pub(crate) fn part2(input: String) {
    let lines: Vec<&str> = input.trim().lines().collect();
    let time = i64::from_str(&lines[0].replace(|c: char| !c.is_ascii_digit(), "")).unwrap();
    let distance = i64::from_str(&lines[1].replace(|c: char| !c.is_ascii_digit(), "")).unwrap();
    let lb = binsearch(time, distance, 0, time/2, true);
    let ub = binsearch(time, distance, time/2, time, false);
    println!("{}", ub - lb + 1);
}
