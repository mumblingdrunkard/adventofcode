#![feature(iter_array_chunks)]

fn to_priority(c: u8) -> usize {
    (c as usize + 58 - 96) % 58
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let part1: usize = input
        .lines()
        .filter_map(|line| {
            let (s0, s1) = line.as_bytes().split_at(line.len() / 2);
            s0.iter()
                .find_map(|b| s1.contains(b).then_some(*b))
                .map(to_priority)
        })
        .sum();

    println!("{part1}");

    let sum: usize = input
        .lines()
        .array_chunks::<3>()
        .filter_map(|g| {
            g[0].bytes().find_map(|b| {
                (g[1].as_bytes().contains(&b) && g[2].as_bytes().contains(&b))
                    .then_some(b)
                    .map(to_priority)
            })
        })
        .sum();

    println!("{sum}");
}
