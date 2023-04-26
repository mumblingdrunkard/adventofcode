use std::{cmp::PartialOrd, ops::RangeInclusive};

trait ContainsRangeInclusive<Item: PartialOrd> {
    fn contains_range_inclusive(&self, range: &RangeInclusive<Item>) -> bool;
}

impl<Item: PartialOrd> ContainsRangeInclusive<Item> for RangeInclusive<Item> {
    fn contains_range_inclusive(&self, range: &RangeInclusive<Item>) -> bool {
        self.start() <= range.start() && self.end() >= range.end()
    }
}

trait OverlapsRangeInclusive<Item: PartialOrd> {
    fn overlaps_range_inclusive(&self, range: &RangeInclusive<Item>) -> bool;
}

impl<Item: PartialOrd> OverlapsRangeInclusive<Item> for RangeInclusive<Item> {
    fn overlaps_range_inclusive(&self, range: &RangeInclusive<Item>) -> bool {
        self.start() <= range.end() && self.end() >= range.start()
    }
}

fn str_to_range(s: &str) -> RangeInclusive<usize> {
    let mut range = s.split('-');
    let start = range.next().unwrap().parse::<usize>().unwrap();
    let end = range.next().unwrap().parse::<usize>().unwrap();
    start..=end
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let count = input
        .lines()
        .filter(|line| {
            let mut ranges = line.split(',').map(str_to_range);
            let a = ranges.next().unwrap();
            let b = ranges.next().unwrap();
            a.contains_range_inclusive(&b) || b.contains_range_inclusive(&a)
        })
        .count();

    println!("{count}");

    let count = input
        .lines()
        .filter(|line| {
            let mut ranges = line.split(',').map(str_to_range);
            let a = ranges.next().unwrap();
            let b = ranges.next().unwrap();
            a.overlaps_range_inclusive(&b)
        })
        .count();

    println!("{count}");
}
