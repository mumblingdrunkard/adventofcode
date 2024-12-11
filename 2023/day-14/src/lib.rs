use std::collections::HashMap;

fn north(rocks: &[impl AsRef<[u8]>]) -> Vec<Vec<u8>> {
    let rows = rocks.len();
    let cols = rocks[0].as_ref().len();
    let mut new_rocks = vec![vec![b'.'; cols]; rows];
    for col in 0..cols {
        let mut start = 0;
        let mut count = 0;
        for row in 0..rows {
            match rocks[row].as_ref()[col] {
                b'.' => {}
                b'O' => {
                    count += 1;
                }
                b'#' => {
                    new_rocks[row][col] = b'#';
                    (start..start + count).for_each(|r| {
                        new_rocks[r][col] = b'O';
                    });
                    count = 0;
                    start = row + 1;
                }
                _ => panic!(),
            }
        }
        (start..start + count).for_each(|r| {
            new_rocks[r][col] = b'O';
        });
    }
    new_rocks
}

fn south(rocks: &[impl AsRef<[u8]>]) -> Vec<Vec<u8>> {
    let rows = rocks.len();
    let cols = rocks[0].as_ref().len();
    let mut new_rocks = vec![vec![b'.'; cols]; rows];
    for col in 0..cols {
        let mut start = rows;
        let mut count = 0;
        for row in (0..rows).rev() {
            match rocks[row].as_ref()[col] {
                b'.' => {}
                b'O' => {
                    count += 1;
                }
                b'#' => {
                    new_rocks[row][col] = b'#';
                    for r in 0..count {
                        new_rocks[start - r - 1][col] = b'O';
                    }
                    count = 0;
                    start = row;
                }
                _ => panic!(),
            }
        }
        for r in 0..count {
            new_rocks[start - r - 1][col] = b'O';
        }
    }
    new_rocks
}

fn west(rocks: &[impl AsRef<[u8]>]) -> Vec<Vec<u8>> {
    let rows = rocks.len();
    let cols = rocks[0].as_ref().len();
    let mut new_rocks = vec![vec![b'.'; cols]; rows];
    for row in 0..rows {
        let mut start = 0;
        let mut count = 0;
        for col in 0..cols {
            match rocks[row].as_ref()[col] {
                b'.' => {}
                b'O' => {
                    count += 1;
                }
                b'#' => {
                    new_rocks[row][col] = b'#';
                    for c in start..start + count {
                        new_rocks[row][c] = b'O';
                    }
                    count = 0;
                    start = col + 1;
                }
                _ => panic!(),
            }
        }
        for c in start..start + count {
            new_rocks[row][c] = b'O';
        }
    }
    new_rocks
}

fn east(rocks: &[impl AsRef<[u8]>]) -> Vec<Vec<u8>> {
    let rows = rocks.len();
    let cols = rocks[0].as_ref().len();
    let mut new_rocks = vec![vec![b'.'; cols]; rows];
    for row in 0..rows {
        let mut start = cols;
        let mut count = 0;
        for col in (0..cols).rev() {
            match rocks[row].as_ref()[col] {
                b'.' => {}
                b'O' => {
                    count += 1;
                }
                b'#' => {
                    new_rocks[row][col] = b'#';
                    for c in 0..count {
                        new_rocks[row][start - 1 - c] = b'O';
                    }
                    count = 0;
                    start = col;
                }
                _ => panic!(),
            }
        }
        for c in 0..count {
            new_rocks[row][start - 1 - c] = b'O';
        }
    }
    new_rocks
}

fn cycle(rocks: &[impl AsRef<[u8]>]) -> Vec<Vec<u8>> {
    let north = north(rocks);
    let west = west(&north);
    let south = south(&west);
    east(&south)
}

fn load(rocks: &[impl AsRef<[u8]>]) -> usize {
    let mut sum = 0;
    for (load, row) in rocks
        .iter()
        .enumerate()
        .map(|(r, row)| (rocks.len() - r, row))
    {
        sum += row.as_ref().iter().filter(|&&b| b == b'O').count() * load;
    }
    sum
}

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(lines: L) -> (i64, i64) {
    // run until steady state?

    let mut rocks = vec![];
    for line in lines {
        rocks.push(line.as_ref().as_bytes().to_vec());
    }

    let north = north(&rocks);
    let part1 = load(&north) as i64;

    let mut rounds = 0;
    let mut seen = HashMap::new();
    let mut rocks = rocks;

    // Find cycle
    while seen.get(&rocks).is_none() {
        let next = cycle(&rocks);
        seen.insert(rocks, rounds);
        rounds += 1;
        rocks = next;
    }

    let cycle_length = rounds - seen.get(&rocks).unwrap();
    for _ in 0..(1000000000 - rounds) % cycle_length {
        rocks = cycle(&rocks);
    }

    (part1, load(&rocks) as i64)
}
