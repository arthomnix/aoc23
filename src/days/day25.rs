use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn parse_input(input: String) -> HashMap<String, HashMap<String, usize>> {
    let mut map: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for line in input.lines() {
        let (source, t) = line.split_once(": ").unwrap();
        for target in t.split_whitespace() {
            if let Some(v) = map.get_mut(source) {
                v.insert(target.to_string(), 1);
            } else {
                map.insert(source.to_string(), HashMap::from([(target.to_string(), 1)]));
            }

            if let Some(v) = map.get_mut(target) {
                v.insert(source.to_string(), 1);
            } else {
                map.insert(target.to_string(), HashMap::from([(source.to_string(), 1)]));
            }
        }
    }

    map
}

fn contract_edge(g: &mut HashMap<String, HashMap<String, usize>>, s: &String, t: &String) {
    let nodes = g.get(t).unwrap().iter().map(|(k, v)| (k.to_string(), *v)).collect::<Vec<_>>();
    for (node, _) in &nodes {
        let e = g.get_mut(node).unwrap();
        let n = *e.get(t).unwrap();
        e.remove(t);
        if s != node {
            if !e.contains_key(s) {
                e.insert(s.to_string(), n);
            } else {
                *e.get_mut(s).unwrap() += n;
            }
        }
    }

    for (node, n) in nodes {
        if s != &node {
            let sc = g.get_mut(s).unwrap();
            if !sc.contains_key(&node) {
                sc.insert(node, n);
            } else {
                *sc.get_mut(&node).unwrap() += n;
            }
        }
    }
    g.remove(t);
}

fn possible_min_cut(mut g: HashMap<String, HashMap<String, usize>>) -> (usize, usize) {
    let mut merges = HashMap::new();
    for u in g.keys() {
        merges.insert(u.to_string(), HashSet::from([u.to_string()]));
    }

    while g.len() > 2 {
        let nodes = g.keys().cloned().collect::<Vec<_>>();
        let s = nodes.choose(&mut thread_rng()).unwrap();
        let v = g.get(s).unwrap().keys().cloned().collect::<Vec<_>>();
        let mut v = vec![];
        for (n, i) in g.get(s).unwrap().iter().map(|(k, v)| (k.to_string(), *v)) {
            for _ in 0..i {
                v.push(n.clone());
            }
        }
        let t = v.choose(&mut thread_rng()).unwrap();
        contract_edge(&mut g, s, t);
        let m = merges.get(t).unwrap().into_iter().cloned().collect::<Vec<_>>();
        merges.remove(t);
        merges.get_mut(s).unwrap().extend(m);
    }

    let n = *g.iter().next().unwrap().1.iter().next().unwrap().1;
    let mut iter = merges.into_iter();
    let n1 = iter.next().unwrap().1.len();
    let n2 = iter.next().unwrap().1.len();
    (n1 * n2, n)
}

pub(crate) fn part1(input: String) {
    let graph = parse_input(input);
    loop {
        let (a, n) = possible_min_cut(graph.clone());
        if n == 3 {
            println!("{a}");
            break;
        }
    }
}

pub(crate) fn part2(input: String) {
    unimplemented!("Advent of Code puzzles do not have a part 2 on Christmas Day!");
}
