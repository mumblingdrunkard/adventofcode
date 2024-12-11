use std::{
    cmp::{max, min},
    collections::BTreeSet,
};

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(lines: L) -> (i64, u64) {
    let mut empty_rows = BTreeSet::new();

    let mut maybe_empty_cols = BTreeSet::new();
    let mut non_empty_cols = BTreeSet::new();

    let mut galaxies = vec![];
    lines.enumerate().for_each(|(row, l)| {
        if l.as_ref().chars().all(|c| c == '.') {
            empty_rows.insert(row);
        }
        l.as_ref().chars().enumerate().for_each(|(col, c)| match c {
            '.' => {
                maybe_empty_cols.insert(col);
            }
            '#' => {
                galaxies.push((row, col));
                non_empty_cols.insert(col);
            }
            _ => unreachable!("There shouldn't be anything but . and # in the input."),
        })
    });

    let empty_cols = maybe_empty_cols
        .difference(&non_empty_cols)
        .copied()
        .collect::<BTreeSet<_>>();

    let (mut part1, mut part2) = (0, 0);
    for (i, &(r1, c1)) in galaxies[..galaxies.len() - 1].iter().enumerate() {
        for &(r2, c2) in galaxies[i + 1..].iter() {
            let (rmin, rmax) = (min(r1, r2), max(r1, r2));
            let (cmin, cmax) = (min(c1, c2), max(c1, c2));

            let rdiff = rmax - rmin;
            let rexpand = empty_rows.range(rmin..rmax).count();
            let cdiff = cmax - cmin;
            let cexpand = empty_cols.range(cmin..cmax).count();

            part1 += rdiff + cdiff + rexpand + cexpand;
            part2 += rdiff + cdiff + 999999 * (rexpand + cexpand);
        }
    }

    (part1 as i64, part2 as u64)
}
