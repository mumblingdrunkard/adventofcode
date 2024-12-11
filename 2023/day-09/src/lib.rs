use rayon::{iter::ParallelBridge, prelude::ParallelIterator};

pub fn solve<T: AsRef<str> + Send, L: Iterator<Item = T> + Send>(lines: L) -> (i64, i64) {
    lines
        .par_bridge()
        .map(|line| {
            let (mut p1, mut p2) = (0, 0);
            let seq = line.as_ref().split_whitespace().map(str::parse::<i64>);
            let mut seq = seq.filter_map(Result::ok).collect::<Vec<_>>();
            let mut m = [1, -1].into_iter().cycle();
            while seq.iter().any(|&v| v != 0) {
                (p1, p2) = (p1 + seq.last().unwrap(), p2 + m.next().unwrap() * seq[0]);
                seq = seq.windows(2).map(|w| w[1] - w[0]).collect();
            }
            (p1, p2)
        })
        .reduce(|| (0, 0), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
}
