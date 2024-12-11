use std::collections::HashMap;

use num::iter::{Range, RangeInclusive};

fn parse_workflow(l: &str) -> (&str, Vec<(char, char, isize, &str)>) {
    let name = l.split('{').next().unwrap();
    let rules = l.trim_start_matches(name);
    let rules = rules.trim_matches(|c| "{}".contains(c));
    let rules = rules.split(',');

    let rules = rules
        .map(|r| {
            let mut split = r.split(':');
            let first = split.next().unwrap();
            let second = split.next();

            let (v, cmp, val, goto);
            if let Some(target) = second {
                goto = target;
                v = first.chars().next().unwrap();
                cmp = first.chars().nth(1).unwrap();
                val = first[2..].parse::<isize>().unwrap();
            } else {
                v = ' ';
                cmp = ' ';
                val = 0;
                goto = first;
            }

            (v, cmp, val, goto)
        })
        .collect();

    (name, rules)
}

fn parse_part(l: &str) -> HashMap<char, isize> {
    let part = l.trim_matches(|c| "{}".contains(c));
    let values = part.split(',');
    values.fold(HashMap::new(), |mut map, val| {
        let c = val.chars().next().unwrap();
        let v = val[2..].parse::<isize>().unwrap();
        map.insert(c, v);
        map
    })
}

fn check(v1: isize, v2: isize, cmp: char) -> bool {
    match cmp {
        '<' => v1 < v2,
        '>' => v1 > v2,
        _ => panic!(),
    }
}

fn run_workflow<'a>(
    workflow: &[(char, char, isize, &'a str)],
    part: &HashMap<char, isize>,
) -> &'a str {
    for &(c, cmp, v2, res) in workflow {
        if c == ' ' {
            return res;
        }

        let v1 = *part.get(&c).unwrap();
        if check(v1, v2, cmp) {
            return res;
        }
    }
    panic!()
}

fn run_parallel_workflow<'a>(
    workflow: &[(char, char, isize, &'a str)],
    part: &HashMap<char, Range<isize>>,
) -> Vec<(&'a str, HashMap<char, Range<isize>>)> {
    let mut parts = vec![];
    let mut part = part.clone();
    for &(c, cmp, v2, res) in workflow {}
    parts
}

fn handle_part<'a>(
    workflows: &'a HashMap<&str, Vec<(char, char, isize, &str)>>,
    part: &HashMap<char, isize>,
) -> &'a str {
    let mut name = "in";
    while let Some(workflow) = workflows.get(name) {
        let next = run_workflow(workflow, part);
        name = match next {
            "A" | "R" => return next,
            next => next,
        }
    }
    panic!()
}

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(lines: L) -> (isize, isize) {
    let lines = lines.map(|l| l.as_ref().to_string()).collect::<Vec<_>>();
    let mut chunks = lines.split(|l| l.is_empty());
    let workflows = chunks.next().unwrap();

    let workflows = workflows
        .iter()
        .map(|l| parse_workflow(l))
        .collect::<HashMap<_, _>>();

    let parts = chunks
        .next()
        .unwrap()
        .iter()
        .map(|s| parse_part(s))
        .collect::<Vec<_>>();

    let mut part1 = 0;
    for part in parts {
        if handle_part(&workflows, &part) == "A" {
            part1 += part.values().sum::<isize>();
        }
    }

    (part1, 0)
}
