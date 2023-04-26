/// Takes a new head and an old tail of a single knot and returns the updated position of the tail
fn follow(new_head: (i32, i32), old_tail: (i32, i32)) -> (i32, i32) {
    match (new_head.0 - old_tail.0, new_head.1 - old_tail.1) {
        (-1..=1, -1..=1) => old_tail, // touching
        (dy, dx) => (old_tail.0 + dy.signum(), old_tail.1 + dx.signum()), // catch up
    }
}

/// Takes a new head and an old tail and returns an updated rope
fn catch_up<const ROPE_LENGTH: usize>(
    new_head: (i32, i32),
    rope: &[(i32, i32); ROPE_LENGTH],
) -> [(i32, i32); ROPE_LENGTH] {
    let mut res = [(0, 0); ROPE_LENGTH];
    let new_values = [new_head]
        .into_iter()
        .chain(rope[1..].iter().scan(new_head, |head, tail| {
            *head = follow(*head, *tail); // each knot catches up to the knot preceding it
            Some(*head)
        }));

    res.iter_mut()
        .zip(new_values)
        .for_each(|(res, val)| *res = val);

    res
}

/// Returns `head`, moved one step in the given direction `dir`
fn step(head: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        'U' => (head.0 + 1, head.1),
        'D' => (head.0 - 1, head.1),
        'R' => (head.0, head.1 + 1),
        _ => (head.0, head.1 - 1),
    }
}

/// Takes an iterator of movements and a rope length and calculates how many
/// positions are visited by the tail of the rope.
fn solve<const ROPE_LENGTH: usize>(movements: impl Iterator<Item = char>) -> usize {
    use std::collections::HashSet;
    movements
        .scan([(0, 0); ROPE_LENGTH], |rope, dir| {
            let head = step(rope[0], dir);
            *rope = catch_up::<ROPE_LENGTH>(head, rope);
            Some(rope[rope.len() - 1]) // yield the updated tail
        })
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let movements = input.lines().flat_map(|line| {
        let line = line.split_whitespace().collect::<Vec<_>>();

        let dir = line[0].chars().next().unwrap();
        let n = line[1].parse().unwrap();

        std::iter::repeat(dir).take(n)
    });

    println!("part 1: {}", solve::<2>(movements.clone()));
    println!("part 2: {}", solve::<10>(movements));
}
