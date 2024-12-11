fn transpose(lines: &[impl AsRef<[u8]>]) -> Vec<Vec<u8>> {
    let rows = lines.len();
    let cols = lines[0].as_ref().len();
    let mut transposed = vec![vec![0; rows]; cols];

    (0..rows).for_each(|i| {
        (0..cols).for_each(|j| {
            transposed[j][i] = lines[i].as_ref()[j];
        });
    });

    transposed
}

fn reflects(line: &[u8], col: usize) -> bool {
    let (left, right) = line.split_at(col);
    left.iter().rev().zip(right.iter()).all(|(&l, &r)| l == r)
}

fn check_pattern(pattern: &[Vec<u8>]) -> (usize, Vec<(usize, usize)>) {
    let mut sum = 0;
    let mut reflections = vec![];
    let cols = pattern[0].len();
    for col in 1..cols {
        if pattern.iter().all(|line| reflects(line, col)) {
            sum += col;
            reflections.push((0, col));
        }
    }

    let pattern = transpose(pattern);
    let cols = pattern[0].len();
    for col in 1..cols {
        if pattern.iter().all(|line| reflects(line, col)) {
            sum += 100 * col;
            reflections.push((col, 0));
        }
    }

    (sum, reflections)
}

fn flip(b: u8) -> u8 {
    match b {
        b'.' => b'#',
        b'#' => b'.',
        _ => panic!(),
    }
}

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(lines: L) -> (i64, i64) {
    let lines = lines
        .map(|l| l.as_ref().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let patterns = lines.split(|l| l.is_empty());
    let mut part1 = 0;
    for pattern in patterns {
        part1 += check_pattern(pattern).0;
    }

    let patterns = lines.split(|l| l.is_empty());
    let mut part2 = 0;
    for pattern in patterns {
        let mut pattern = pattern.iter().map(Vec::to_owned).collect::<Vec<_>>();

        // old line of reflection
        let (r, c) = check_pattern(&pattern).1[0];

        let cols = pattern[0].len();
        let rows = pattern.len();

        let mut found_smudge = false;

        'outer: for col in 0..cols {
            for row in 0..rows {
                pattern[row][col] = flip(pattern[row][col]);
                // new line(s) of reflection
                let (_, reflections) = check_pattern(&pattern);
                for (dr, dc) in reflections {
                    if dr != 0 && dr != r {
                        part2 += 100 * dr;
                        found_smudge = true;
                        break 'outer;
                    } else if dc != 0 && dc != c {
                        part2 += dc;
                        found_smudge = true;
                        break 'outer;
                    }
                }
                pattern[row][col] = flip(pattern[row][col]);
            }
        }

        if !found_smudge {
            println!("No smudge found");
        }
    }

    (part1 as i64, part2 as i64)
}
