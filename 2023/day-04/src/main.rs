use std::collections::HashSet;

fn count_winnings(s: &str) -> usize {
    let mut sets = s.split(": ").nth(1).unwrap().split(" | ");
    let winning = sets
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<HashSet<_>>();
    let my_numbers = sets
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<HashSet<_>>();
    winning.intersection(&my_numbers).count()
}

fn main() {
    let mut winnings = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .map(|s| (count_winnings(&s), 1usize))
        .collect::<Vec<_>>();
    for i in 0..winnings.len() {
        for j in i + 1..i + winnings[i].0 + 1 {
            winnings[j].1 += winnings[i].1;
        }
    }
    let mut points = 0;
    let mut sum = 0;
    for winning in winnings {
        points += (1 << winning.0) >> 1;
        sum += winning.1;
    }
    println!("{points} | {sum}");
}
