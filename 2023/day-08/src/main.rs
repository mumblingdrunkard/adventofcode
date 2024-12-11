use std::collections::{HashMap, HashSet};

fn main() {
    // Ugly parsing code
    let mut lines = std::io::stdin().lines().map_while(Result::ok);
    let seq = lines.next().unwrap();
    lines.next().unwrap();

    let network = lines
        .map(|line| {
            let id = line.split(" = ").next().unwrap().to_owned();
            let lr = line.split(" = ").nth(1).unwrap().to_owned();
            let l = lr
                .split(", ")
                .next()
                .unwrap()
                .trim_start_matches('(')
                .to_owned();
            let r = lr
                .split(", ")
                .nth(1)
                .unwrap()
                .trim_end_matches(')')
                .to_owned();
            (id, (l, r))
        })
        .collect::<HashMap<_, _>>();

    // Iterate until ZZZ
    let mut curr = "AAA";
    let mut step = (0..).flat_map(|_| seq.chars());
    let mut count = 0;
    while curr != "ZZZ" {
        count += 1;
        let step = step.next().unwrap();
        let (l, r) = network.get(curr).unwrap();
        match step {
            'L' => curr = l,
            'R' => curr = r,
            _ => panic!(),
        }
    }
    println!("part 1: {count}");

    // All ending in A
    let mut curr = network
        .keys()
        .filter(|s| s.ends_with('A'))
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut step = (0..).flat_map(|_| seq.chars());
    let mut periods = HashSet::new();
    // While we still have paths to check
    while !curr.is_empty() {
        count += 1;
        let step = step.next().unwrap();
        let new_curr = curr
            .into_iter()
            .map(|curr| {
                let (l, r) = network.get(curr).unwrap();
                match step {
                    'L' => l,
                    'R' => r,
                    _ => panic!(),
                }
            })
            .filter(|s| {
                if s.ends_with('Z') {
                    // Insert period...
                    periods.insert(count);
                }
                // And filter out
                !s.ends_with('Z')
            })
            .collect::<Vec<_>>();
        curr = new_curr;
    }

    // This totally shouldn't be working and it's a dumb assumption that the first occurence of a Z leads back to the same path

    println!(
        "part 2: {}",
        periods.into_iter().fold(1u64, num::integer::lcm)
    );
}
