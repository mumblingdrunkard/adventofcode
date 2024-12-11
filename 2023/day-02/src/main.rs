use std::{collections::HashMap, io::stdin};

fn main() {
    let lines = stdin().lines().map_while(Result::ok);
    let mut sum = 0;
    let mut power_sum = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        };

        let mut split = line.trim().split(": ");
        let game_id = split.next().unwrap();
        let id = game_id
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let sets = split.next().unwrap().split("; ");

        let mut highest_seen = [("red", 0), ("green", 0), ("blue", 0)]
            .into_iter()
            .collect::<HashMap<_, usize>>();

        for set in sets {
            let entries = set.split(", ");
            for entry in entries {
                let mut split = entry.split_whitespace();
                let cnt = split.next().unwrap().parse::<usize>().unwrap();
                let col = split.next().unwrap();
                let entry = highest_seen.get_mut(col).unwrap();
                *entry = std::cmp::max(*entry, cnt);
            }
        }

        let red = *highest_seen.get("red").unwrap();
        let green = *highest_seen.get("green").unwrap();
        let blue = *highest_seen.get("blue").unwrap();

        let red_ok = red <= 12;
        let green_ok = green <= 13;
        let blue_ok = blue <= 14;

        if red_ok && green_ok && blue_ok {
            sum += id;
        }

        let power = red * green * blue;
        power_sum += power;
    }

    println!("{sum}");
    println!("{power_sum}");
}
