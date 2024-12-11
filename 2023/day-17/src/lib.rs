use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(lines: L) -> (usize, usize) {
    let map = lines
        .map(|l| {
            l.as_ref()
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = map[0].len() as isize;
    let height = map.len() as isize;
    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();
    const DIR: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    q.push(Reverse((0, ((0, 0), 0, 0, 1))));
    q.push(Reverse((0, ((0, 0), 1, 0, 1))));

    q.push(Reverse((0, ((0, 0), 0, 0, 2))));
    q.push(Reverse((0, ((0, 0), 1, 0, 2))));
    let mut part1 = 0;
    let mut part2 = 0;
    while let Some(Reverse((cost, state))) = q.pop() {
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);
        let ((x, y), dir, cnt, part) = state;
        if x == width - 1 && y == height - 1 && cnt <= 3 && part1 == 0 && part == 1 {
            part1 = cost;
        }
        if x == width - 1 && y == height - 1 && (4..=10).contains(&cnt) && part2 == 0 && part == 2 {
            part2 = cost;
        }

        if (part == 1 && part1 != 0) || (part == 2 && part2 != 0) {
            continue;
        }

        let mut dirs = vec![];
        if part == 1 {
            dirs.push((dir + 1) & 3);
            dirs.push((dir + 3) & 3);
            if cnt < 3 {
                dirs.push(dir);
            }
        } else if part == 2 {
            if cnt >= 4 {
                dirs.push((dir + 1) & 3);
                dirs.push((dir + 3) & 3);
            }
            if cnt < 10 {
                dirs.push(dir);
            }
        }

        for d in dirs {
            let (dx, dy) = DIR[d];
            let (x, y) = (x + dx, y + dy);
            if !(0..width).contains(&x) || !(0..height).contains(&y) {
                continue;
            }
            let cnt = if dir == d { cnt + 1 } else { 1 };
            let cost = cost + map[y as usize][x as usize];
            q.push(Reverse((cost, ((x, y), d, cnt, part))));
        }
    }

    (part1, part2)
}
