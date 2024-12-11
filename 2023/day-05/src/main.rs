use rayon::{
    self,
    prelude::{IntoParallelRefIterator, ParallelIterator},
};
use std::collections::BTreeMap;

fn convert(n: u64, map: &BTreeMap<u64, (u64, u64)>) -> u64 {
    match map.range(..n + 1).last() {
        Some((&src, &(dst, len))) => {
            let offset = n - src;
            if offset < len {
                dst + offset
            } else {
                n
            }
        }
        _ => n,
    }
}

fn convert_many(n: u64, maps: &[BTreeMap<u64, (u64, u64)>]) -> u64 {
    let mut prev = n;
    for map in maps {
        prev = convert(prev, map);
    }
    prev
}

fn convert_range(range: (u64, u64), map: &BTreeMap<u64, (u64, u64)>) -> Vec<(u64, u64)> {
    let mut v = vec![];
    let (mut start, len) = range;
    let end = start + len;

    let base = match map.range(..start + 1).next() {
        Some((&src, _)) => src,
        _ => 0,
    };

    let mappings = map.range(base..end);
    for (&src, &(dst, n)) in mappings {
        // The range we can map
        let range = src..src + n;

        if start < range.start {
            let end = std::cmp::min(end, range.start);
            v.push((start, end - start));
            start = end;
        }

        if range.contains(&start) {
            let end = std::cmp::min(end, range.end);
            v.push((start - range.start + dst, end - start));
            start = end;
        }
    }

    v
}

fn solve() {
    let lines = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let mut chunks = lines.split(|s| s.is_empty());

    let seeds = chunks.next().unwrap()[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(str::parse::<u64>)
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let maps = chunks
        .map(|chunk| {
            chunk
                .iter()
                .skip(1)
                .map(|line| {
                    let mapping = line
                        .split_whitespace()
                        .map(str::parse::<u64>)
                        .map_while(Result::ok)
                        .collect::<Vec<_>>();
                    (mapping[1], (mapping[0], mapping[2]))
                })
                .collect::<BTreeMap<_, _>>()
        })
        .collect::<Vec<_>>();

    let locations = seeds
        .iter()
        .map(|&seed| convert_many(seed, &maps))
        .collect::<Vec<_>>();

    let mut ranges = seeds.chunks(2).map(|v| (v[0], v[1])).collect::<Vec<_>>();

    let part2_brute = ranges
        .par_iter()
        .flat_map(|&(start, len)| start..start + len)
        .map(|seed| convert_many(seed, &maps))
        .min()
        .unwrap();

    for map in &maps {
        let mut new_ranges = vec![];
        for range in ranges {
            new_ranges.extend_from_slice(&convert_range(range, map));
        }
        ranges = new_ranges;
    }

    println!("part 1: {}", locations.iter().copied().min().unwrap());
    println!(
        "part 2: {}",
        ranges.iter().map(|(start, _)| start).min().unwrap()
    );

    println!("part 2 brute: {part2_brute}");
}

fn main() {
    solve();
}
