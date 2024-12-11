use std::io::stdin;

use regex::Regex;

fn str_to_u32(s: &str) -> u32 {
    match s {
        "eno" | "one" | "1" => 1,
        "owt" | "two" | "2" => 2,
        "eerht" | "three" | "3" => 3,
        "ruof" | "four" | "4" => 4,
        "evif" | "five" | "5" => 5,
        "xis" | "six" | "6" => 6,
        "neves" | "seven" | "7" => 7,
        "thgie" | "eight" | "8" => 8,
        "enin" | "nine" | "9" => 9,
        _ => 0,
    }
}

fn part2(s: &str) -> u32 {
    let re = Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let first = re.find(s).map(|m| m.as_str()).map(str_to_u32).unwrap();

    let t = s.chars().rev().collect::<String>();
    let er = Regex::new("[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let last = er.find(&t).map(|m| m.as_str()).map(str_to_u32).unwrap();

    first * 10 + last
}

fn part1(s: &str) -> u32 {
    let to_digit = |c: char| c.to_digit(10);
    let first = s.chars().find_map(to_digit).unwrap();
    let last = s.chars().rev().find_map(to_digit).unwrap();
    first * 10 + last
}

fn main() {
    let sums = stdin()
        .lines()
        .map_while(Result::ok)
        .filter(|s| !s.is_empty())
        .map(|s| (part1(&s), part2(&s)))
        .fold((0, 0), |(s1, s2), (v1, v2)| (s1 + v1, s2 + v2));

    println!("{sums:?}");
}
