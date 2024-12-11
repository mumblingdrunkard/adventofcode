use std::collections::VecDeque;

fn check(
    cave: &[u8],
    rows: usize,
    cols: usize,
    beams: &mut VecDeque<(isize, isize, isize, isize)>,
    entered_from: &mut [i8],
) -> usize {
    while let Some((row, col, dr, dc)) = beams.pop_front() {
        if !(0..rows as isize).contains(&row) || !(0..cols as isize).contains(&col) {
            continue;
        }

        let dir = match (dr, dc) {
            (-1, 0) => 1,
            (1, 0) => 4,
            (0, -1) => 2,
            (0, 1) => 8,
            _ => unsafe { std::hint::unreachable_unchecked() },
        };

        if entered_from[row as usize * cols + col as usize] & dir != 0 {
            continue;
        }
        entered_from[row as usize * cols + col as usize] |= dir;

        match cave[row as usize * cols + col as usize] {
            b'.' => beams.push_back((row + dr, col + dc, dr, dc)),
            b'/' => match (dr, dc) {
                (0, dc) => beams.push_back((row - dc, col, -dc, 0)),
                (dr, 0) => beams.push_back((row, col - dr, 0, -dr)),
                _ => unsafe { std::hint::unreachable_unchecked() },
            },
            b'\\' => match (dr, dc) {
                (0, dc) => beams.push_back((row + dc, col, dc, 0)),
                (dr, 0) => beams.push_back((row, col + dr, 0, dr)),
                _ => unsafe { std::hint::unreachable_unchecked() },
            },
            b'-' => match (dr, dc) {
                (0, 1) | (0, -1) => beams.push_back((row + dr, col + dc, dr, dc)),
                (1, 0) | (-1, 0) => {
                    beams.push_back((row, col - 1, 0, -1));
                    beams.push_back((row, col + 1, 0, 1));
                    entered_from[row as usize * cols + col as usize] |= 0b0101;
                }
                _ => unsafe { std::hint::unreachable_unchecked() },
            },
            b'|' => match (dr, dc) {
                (0, 1) | (0, -1) => {
                    beams.push_back((row - 1, col, -1, 0));
                    beams.push_back((row + 1, col, 1, 0));
                    entered_from[row as usize * cols + col as usize] |= 0b1010;
                }
                (1, 0) | (-1, 0) => beams.push_back((row + dr, col + dc, dr, dc)),
                _ => unsafe { std::hint::unreachable_unchecked() },
            },
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    let energized = entered_from.iter().filter(|i| i.is_positive()).count();
    entered_from.fill(0);
    energized
}

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(lines: L) -> (usize, usize) {
    let cave = lines
        .map(|l| l.as_ref().as_bytes().to_vec())
        .collect::<Vec<_>>();

    let rows = cave.len();
    let cols = cave[0].len();

    let cave = cave
        .into_iter()
        .flat_map(|r| r.into_iter())
        .collect::<Vec<_>>();

    let mut entered_from = vec![0; rows * cols];
    let mut beams = VecDeque::new();
    beams.push_back((0, 0, 0, 1));

    let part1 = check(&cave, rows, cols, &mut beams, &mut entered_from);

    let mut part2 = 0;
    for r in 0..rows as isize {
        beams.push_back((r, 0, 0, 1));
        part2 = std::cmp::max(
            part2,
            check(&cave, rows, cols, &mut beams, &mut entered_from),
        );
        beams.push_back((r, cols as isize - 1, 0, -1));
        part2 = std::cmp::max(
            part2,
            check(&cave, rows, cols, &mut beams, &mut entered_from),
        );
    }

    for c in 0..cols as isize {
        beams.push_back((0, c, 1, 0));
        part2 = std::cmp::max(
            part2,
            check(&cave, rows, cols, &mut beams, &mut entered_from),
        );
        beams.push_back((rows as isize - 1, c, -1, 0));
        part2 = std::cmp::max(
            part2,
            check(&cave, rows, cols, &mut beams, &mut entered_from),
        );
    }

    (part1, part2)
}
