use std::collections::VecDeque;
use std::str::FromStr;

fn parse_histories(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace()
            .map(|n| i32::from_str(n).unwrap())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<Vec<_>>>()
}

fn process_diffs(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut diffs = vec![history];

    loop {
        let     prev = diffs.last().unwrap();
        let mut next = Vec::with_capacity(prev.len() - 1);
        let mut same = true;
        for w in prev.windows(2) {
            let d = w[1] - w[0];
            if let Some(l) = next.last() {
                if *l != d {
                    same = false;
                }
            }
            next.push(d);
        }
        diffs.push(next);
        if same {
            break;
        }
    }

    diffs
}

pub(crate) fn part1(input: String) {
    let histories = parse_histories(input);
    println!("{}", histories.into_iter().map(|history| {
        let mut diffs = process_diffs(history);

        let last = diffs.last_mut().unwrap();
        let n = *last.last().unwrap();
        last.push(n);

        for i in (1..diffs.len()).rev() {
            let p = &diffs[i];
            let n = *p.last().unwrap();
            let d = &mut diffs[i - 1];
            let m = *d.last().unwrap();
            d.push(n + m);
        }

        *diffs[0].last().unwrap()
    }).sum::<i32>());
}

pub(crate) fn part2(input: String) {
    let histories = parse_histories(input);
    println!("{}", histories.into_iter().map(|history| {
        let mut diffs = process_diffs(history)
            .into_iter()
            .map(VecDeque::from)
            .collect::<Vec<VecDeque<_>>>();

        let last = diffs.last_mut().unwrap();
        last.push_front(last[0]);

        for i in (1..diffs.len()).rev() {
            let p = &diffs[i];
            let n = p[0];
            let d = &mut diffs[i - 1];
            let m = d[0];
            d.push_front(m - n);
        }

        diffs[0][0]
    }).sum::<i32>())
}
