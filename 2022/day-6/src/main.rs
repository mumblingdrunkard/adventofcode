use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let chars = input.chars().collect::<Vec<_>>();

    let ans = chars
        .windows(4)
        .enumerate()
        .find_map(|(i, w)| {
            let set = w.iter().collect::<HashSet<_>>();
            (set.len() == w.len()).then_some(i + w.len())
        })
        .unwrap();

    println!("{ans}");

    let ans = chars
        .windows(14)
        .enumerate()
        .find_map(|(i, w)| {
            let set = w.iter().collect::<HashSet<_>>();
            (set.len() == w.len()).then_some(i + w.len())
        })
        .unwrap();

    println!("{ans}");
}
