use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn get_rating_by_type(&self, rtype: RatingType) -> i64 {
        match rtype {
            RatingType::X => self.x,
            RatingType::M => self.m,
            RatingType::A => self.a,
            RatingType::S => self.s,
        }
    }

    fn get_rating_mut_by_type(&mut self, rtype: RatingType) -> &mut i64 {
        match rtype {
            RatingType::X => &mut self.x,
            RatingType::M => &mut self.m,
            RatingType::A => &mut self.a,
            RatingType::S => &mut self.s,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Comparator {
    LessThan,
    GreaterThan,
}

impl Comparator {
    fn apply(&self, rtype: RatingType, part: Part, n: i64) -> bool {
        match self {
            Comparator::LessThan => part.get_rating_by_type(rtype) < n,
            Comparator::GreaterThan => part.get_rating_by_type(rtype) > n,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum RatingType {
    X,
    M,
    A,
    S,
}

impl FromStr for RatingType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(RatingType::X),
            "m" => Ok(RatingType::M),
            "a" => Ok(RatingType::A),
            "s" => Ok(RatingType::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Conditional(RatingType, Comparator, i64, String),
    Unconditional(String),
}

impl Rule {
    fn opposite(&self) -> Self {
        match self {
            Rule::Unconditional(s) => Rule::Unconditional(s.clone()),
            Rule::Conditional(rtype, Comparator::GreaterThan, n, s) => Rule::Conditional(*rtype, Comparator::LessThan, *n + 1, s.clone()),
            Rule::Conditional(rtype, Comparator::LessThan, n, s) => Rule::Conditional(*rtype, Comparator::GreaterThan, *n - 1, s.clone()),
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq)]
enum WorkflowResult {
    Accept,
    Reject,
    Jump(String),
}

impl FromStr for WorkflowResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(WorkflowResult::Accept),
            "R" => Ok(WorkflowResult::Reject),
            s => Ok(WorkflowResult::Jump(s.to_string())),
        }
    }
}

impl Workflow {
    fn run(&self, part: Part) -> WorkflowResult {
        for rule in &self.rules {
            match rule {
                Rule::Unconditional(jump_to) => return WorkflowResult::from_str(jump_to).unwrap(),
                Rule::Conditional(rtype, comp, n, jump_to) => if comp.apply(*rtype, part, *n) {
                    return WorkflowResult::from_str(jump_to).unwrap();
                },
            }
        }

        panic!("Workflow ended without accepting or rejecting!");
    }
}

fn run_workflows(workflows: &HashMap<String, Workflow>, part: Part) -> i64 {
    let mut result = WorkflowResult::Jump("in".to_string());
    while let WorkflowResult::Jump(s) = &result {
        let workflow = workflows.get(s).unwrap();
        result = workflow.run(part);
    }

    match result {
        WorkflowResult::Accept => part.x + part.m + part.a + part.s,
        WorkflowResult::Reject => 0,
        WorkflowResult::Jump(_) => unreachable!(),
    }
}

fn parse_input(input: String) -> (HashMap<String, Workflow>, Vec<Part>) {
    let regex = Regex::new(r"^(?P<id>.*?)\{(?P<rules>.*?)}$").unwrap();
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let mut workflows_map = HashMap::new();
    for line in workflows.lines() {
        let workflows_caps = regex.captures(line).unwrap();
        let id = &workflows_caps["id"];
        let rules = &workflows_caps["rules"];

        let mut rv = vec![];
        for rule in rules.split(",") {
            if !rule.contains(":") {
                rv.push(Rule::Unconditional(rule.to_string()));
            } else {
                let (c, t) = rule.split_once(":").unwrap();
                let rtype = RatingType::from_str(&c[0..1]).unwrap();
                let condtype = if c.contains(">") { Comparator::GreaterThan } else { Comparator::LessThan };
                let n = i64::from_str(&c.chars().filter(char::is_ascii_digit).collect::<String>()).unwrap();
                rv.push(Rule::Conditional(rtype, condtype, n, t.to_string()));
            }
        }

        workflows_map.insert(id.to_string(), Workflow {
            rules: rv,
        });
    }

    let regex = Regex::new(r"^\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)}$").unwrap();
    let mut parts_v = vec![];
    for line in parts.lines() {
        let part_caps = regex.captures(line).unwrap();
        let x = i64::from_str(&part_caps["x"]).unwrap();
        let m = i64::from_str(&part_caps["m"]).unwrap();
        let a = i64::from_str(&part_caps["a"]).unwrap();
        let s = i64::from_str(&part_caps["s"]).unwrap();
        parts_v.push(Part { x, m, a, s });

    }

    (workflows_map, parts_v)
}

pub(crate) fn part1(input: String) {
    let (workflows, parts) = parse_input(input);
    println!("{}", parts.into_iter().map(|p| run_workflows(&workflows, p)).sum::<i64>());
}

#[derive(Debug, Clone)]
struct RuleNode {
    rule: Rule,
    prev: Option<Rc<RuleNode>>,
}

fn clone_option_rc<T>(rc: &Option<Rc<T>>) -> Option<Rc<T>> {
    rc.as_ref().map(|r| Rc::clone(r))
}

fn build_tree(workflows: &HashMap<String, Workflow>, start: &str, mut prev: Option<Rc<RuleNode>>) -> Vec<RuleNode> {
    let workflow = &workflows[start];
    let mut v = vec![];

    for rule in &workflow.rules {
        match rule {
            Rule::Conditional(rtype, comp, n, id) => {
                let result = WorkflowResult::from_str(id).unwrap();
                let p = RuleNode {
                    rule: rule.clone(),
                    prev: clone_option_rc(&prev),
                };
                if result == WorkflowResult::Accept {
                    v.push(p.clone());
                } else if let WorkflowResult::Jump(target) = result {
                    let mut result = build_tree(workflows, &target, Some(Rc::new(p)));
                    v.append(&mut result);
                }

                prev = Some(Rc::new(RuleNode {
                    rule: rule.opposite(),
                    prev: clone_option_rc(&prev),
                }))
            },
            Rule::Unconditional(id) => {
                let result = WorkflowResult::from_str(id).unwrap();
                let p = RuleNode {
                    rule: rule.clone(),
                    prev: clone_option_rc(&prev),
                };
                if result == WorkflowResult::Accept {
                    v.push(p.clone());
                } else if let WorkflowResult::Jump(target) = result {
                    let mut result = build_tree(workflows, &target, Some(Rc::new(p)));
                    v.append(&mut result);
                }
            },
        }
    }

    v
}

pub(crate) fn part2(input: String) {
    let (workflows, _) = parse_input(input);
    let n = build_tree(&workflows, "in", None).into_iter().map(|end| {
        let mut mins = Part { x: 0, m: 0, a: 0, s: 0 };
        let mut maxs = Part { x: 4001, m: 4001, a: 4001, s: 4001 };
        let mut prev = Some(Rc::new(end));
        while let Some(p) = prev {
            match p.rule {
                Rule::Conditional(rtype, Comparator::LessThan, n, _) => {
                    let max = maxs.get_rating_by_type(rtype);
                    *maxs.get_rating_mut_by_type(rtype) = max.min(n);
                },
                Rule::Conditional(rtype, Comparator::GreaterThan, n, _) => {
                    let min = mins.get_rating_by_type(rtype);
                    *mins.get_rating_mut_by_type(rtype) = min.max(n);
                },
                _ => {},
            }
            prev = clone_option_rc(&p.prev);
        }

        let dx = maxs.x - mins.x - 1;
        let dm = maxs.m - mins.m - 1;
        let da = maxs.a - mins.a - 1;
        let ds = maxs.s - mins.s - 1;

        dx * dm * da * ds
    }).sum::<i64>();
    println!("{n}");
}
