use std::collections::{HashMap, VecDeque};
use std::convert::identity;
use std::fmt::Debug;

trait Module: Debug {
    fn receive_pulse(&mut self, is_high: bool, sender: &str) -> Option<bool>;
    fn get_destination_ids(&self) -> &Vec<String>;
    fn is_original_state(&self) -> bool;
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct BroadcastModule {
    destinations: Vec<String>,
}

impl Module for BroadcastModule {
    fn receive_pulse(&mut self, is_high: bool, _sender: &str) -> Option<bool> {
        Some(is_high)
    }

    fn get_destination_ids(&self) -> &Vec<String> {
        &self.destinations
    }

    fn is_original_state(&self) -> bool {
        true
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct FlipFlopModule {
    destinations: Vec<String>,
    state: bool,
}

impl Module for FlipFlopModule {
    fn receive_pulse(&mut self, is_high: bool, _sender: &str) -> Option<bool> {
        if !is_high {
            self.state = !self.state;
            Some(self.state)
        } else {
            None
        }
    }

    fn get_destination_ids(&self) -> &Vec<String> {
        &self.destinations
    }

    fn is_original_state(&self) -> bool {
        !self.state
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ConjunctionModule {
    destinations: Vec<String>,
    state: HashMap<String, bool>,
}

impl Module for ConjunctionModule {
    fn receive_pulse(&mut self, is_high: bool, sender: &str) -> Option<bool> {
        self.state.insert(sender.to_string(), is_high);
        if self.state.values().cloned().all(identity) {
            Some(false)
        } else {
            Some(true)
        }
    }

    fn get_destination_ids(&self) -> &Vec<String> {
        &self.destinations
    }

    fn is_original_state(&self) -> bool {
        self.state.values().all(|&b| !b)
    }
}

fn parse_input(input: String) -> (HashMap<String, Box<dyn Module>>, HashMap<String, Vec<String>>) {
    let mut v: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut conjunctions: HashMap<String, ConjunctionModule> = HashMap::new();
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (name, targets) = line.split_once(" -> ").unwrap();
        let targets = targets.split(", ").collect::<Vec<&str>>();
        let (typeid, name) = if name.starts_with("%") {
            ("%", &name[1..])
        } else if name.starts_with("&") {
            ("&", &name[1..])
        } else {
            ("", name)
        };

        for target in &targets {
            if !inputs.contains_key(&target.to_string()) {
                inputs.insert(target.to_string(), vec![name.to_string()]);
            } else {
                inputs.get_mut(&target.to_string()).unwrap().push(name.to_string());
            }
        }

        let destinations = targets.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>();

        match typeid {
            "%" => {
                let module = FlipFlopModule {
                    destinations,
                    state: false,
                };
                v.insert(name.to_string(), Box::new(module));
            },
            "" => {
                let module = BroadcastModule {
                    destinations,
                };
                v.insert(name.to_string(), Box::new(module));
            }
            "&" => {
                let module = ConjunctionModule {
                    destinations,
                    state: HashMap::new(),
                };
                conjunctions.insert(name.to_string(), module);
            }
            _ => unreachable!(),
        }
    }

    for (name, mut conjunction) in conjunctions.into_iter() {
        let in_ids = inputs.get(&name).unwrap();
        for id in in_ids {
            conjunction.state.insert(id.to_string(), false);
        }
        v.insert(name, Box::new(conjunction));
    }

    (v, inputs)
}

struct Pulse {
    is_high: bool,
    source: String,
    destination: String,
}

fn handle_pulses(modules: &mut HashMap<String, Box<dyn Module>>, pulses: &mut VecDeque<Pulse>, looking_for: &[String]) -> (usize, usize, Vec<bool>) {
    pulses.push_back(Pulse {
        is_high: false,
        source: "button".to_string(),
        destination: "broadcaster".to_string(),
    });

    let mut low_sent = 0;
    let mut high_sent = 0;
    let mut v = vec![false; looking_for.len()];

    while let Some(pulse) = pulses.pop_front() {
        for (idx, s) in looking_for.iter().enumerate() {
            if &pulse.source == s && pulse.is_high {
                v[idx] = true;
            }
        }

        if pulse.is_high {
            high_sent += 1;
        } else {
            low_sent += 1;
        }

        if !modules.contains_key(&pulse.destination) {
            continue;
        }

        let destination_module = modules.get_mut(&pulse.destination).unwrap();
        let new_pulse = destination_module.receive_pulse(pulse.is_high, &pulse.source);
        if let Some(is_high) = new_pulse {
            for dest in destination_module.get_destination_ids() {
                pulses.push_back(Pulse {
                    is_high,
                    source: pulse.destination.clone(),
                    destination: dest.clone(),
                })
            }
        }
    }

    (low_sent, high_sent, v)
}

pub(crate) fn part1(input: String) {
    let (mut modules, _) = parse_input(input);
    let mut pulses = VecDeque::new();
    let mut values = vec![];

    for i in 0..1000 {
        let (low_sent, high_sent, _) = handle_pulses(&mut modules, &mut pulses, &[]);
        values.push((low_sent, high_sent));
        if modules.values().all(|m| m.is_original_state()) {
            break;
        }
    }


    let repeats = 1000 / values.len();
    let remaining = 1000 % values.len();
    let (mut total_low_sent, mut total_high_sent) = values.iter().cloned().reduce(|(l1, h1), (l2, h2)| (l1 + l2, h1 + h2)).unwrap();
    total_low_sent *= repeats;
    total_high_sent *= repeats;
    if remaining > 0 {
        let (additional_low_sent, additional_high_sent) = values[0..remaining].iter().cloned().reduce(|(l1, h1), (l2, h2)| (l1 + l2, h1 + h2)).unwrap();
        total_low_sent += additional_low_sent;
        total_high_sent += additional_high_sent;
    }

    dbg!(repeats);

    println!("{}", total_high_sent * total_low_sent);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a.rem_euclid(b))
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

pub(crate) fn part2(input: String) {
    let (mut modules, inputs) = parse_input(input);
    let mut pulses = VecDeque::new();
    let looking_for = inputs.get(&inputs.get("rx").unwrap()[0]).unwrap();
    let mut periods: Vec<u64> = vec![0; looking_for.len()];
    let mut presses = 0;
    loop {
        presses += 1;
        let (_, _, v) = handle_pulses(&mut modules, &mut pulses, looking_for);
        for (idx, b) in v.into_iter().enumerate() {
            if b && periods[idx] == 0 {
                periods[idx] = presses;
            }
        }
        if periods.iter().all(|&p| p != 0) {
            break;
        }
    }

    println!("{}", periods.into_iter().reduce(lcm).unwrap());
}
