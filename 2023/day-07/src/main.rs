use std::collections::HashMap;

enum HandKind {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn get_kind(a: &str) -> HandKind {
    let mut m = HashMap::new();
    a.chars().for_each(|c| {
        *m.entry(c).or_insert(0) += 1;
    });

    if m.len() == 1 {
        HandKind::FiveOfAKind
    } else if m.len() == 2 {
        let mut v = m.values().copied().collect::<Vec<_>>();
        v.sort();
        match v[..] {
            [2, 3] => match m.contains_key(&'J') {
                true => HandKind::FiveOfAKind,
                false => HandKind::FullHouse,
            },
            [1, 4] => match m.contains_key(&'J') {
                true => HandKind::FiveOfAKind,
                false => HandKind::FourOfAKind,
            },
            _ => panic!(),
        }
    } else if m.len() == 4 {
        match m.get(&'J') {
            Some(1 | 2) => HandKind::ThreeOfAKind,
            _ => HandKind::OnePair,
        }
    } else if m.len() == 3 {
        let mut v = m.values().copied().collect::<Vec<_>>();
        v.sort();
        match v[..] {
            [1, 1, 3] => match m.get(&'J') {
                Some(1 | 3) => HandKind::FourOfAKind,
                _ => HandKind::ThreeOfAKind,
            },
            [1, 2, 2] => match m.get(&'J') {
                Some(2) => HandKind::FourOfAKind,
                Some(1) => HandKind::FullHouse,
                _ => HandKind::TwoPair,
            },
            _ => panic!(),
        }
    } else if m.len() == 5 {
        match m.get(&'J') {
            Some(1) => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    } else {
        panic!()
    }
}

fn get_strength(a: char) -> usize {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!(),
    }
}

fn order_cards(a: char, b: char) -> std::cmp::Ordering {
    get_strength(a).cmp(&get_strength(b))
}

fn order_hands(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    let kind_a = get_kind(a) as usize;
    let kind_b = get_kind(b) as usize;
    if kind_a == kind_b {
        for (a, b) in a.chars().zip(b.chars()) {
            if a != b {
                return order_cards(a, b);
            }
        }
        Equal
    } else {
        kind_a.cmp(&kind_b)
    }
}

fn main() {
    let mut hands = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .map(|s| {
            let mut line = s.split_whitespace();
            let hand = line.next().unwrap().to_owned();
            let bid = line.next().unwrap().parse::<u32>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by(|(a, _), (b, _)| order_hands(a, b));

    let mut sum = 0;

    for (rank, (_, bid)) in hands.into_iter().enumerate() {
        sum += bid as usize * (rank + 1);
    }

    println!("{sum}");
}
