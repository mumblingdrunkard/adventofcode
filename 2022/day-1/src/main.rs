use std::{collections::BTreeSet, fs::read_to_string};
fn main() {
    let input = read_to_string("./input").unwrap();
    let elves = input
        .split("\n\n")
        .map(|elf| elf.lines().filter_map(|s| s.parse::<i32>().ok()).sum())
        .collect::<BTreeSet<_>>();
    println!("first star: {}", elves.iter().rev().take(3).sum::<i32>());
    println!("second star: {}", elves.iter().rev().take(3).sum::<i32>());
}
