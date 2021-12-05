use std::{io, time};

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

struct Board {
    board: [(i32, bool); BOARD_SIZE],
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = (i32, bool);

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.board[index.0 * BOARD_WIDTH + index.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.board[index.0 * BOARD_WIDTH + index.1]
    }
}

impl Board {
    fn check(&self) -> bool {
        let row_wise = (0..BOARD_HEIGHT)
            .map(|r| (0..BOARD_WIDTH).map(|c| self[(r, c)].1).all(|v| v == true))
            .any(|v| v == true);

        let column_wise = (0..BOARD_WIDTH)
            .map(|c| (0..BOARD_HEIGHT).map(|r| self[(r, c)].1).all(|v| v == true))
            .any(|v| v == true);

        column_wise || row_wise
    }

    fn mark(&mut self, n: i32) {
        self.board
            .iter_mut()
            .for_each(|(value, marked)| *marked = *marked || *value == n);
    }

    fn score(&self, n: i32) -> i32 {
        self.board
            .iter()
            .map(|(value, marked)| match marked {
                false => *value * n,
                true => 0,
            })
            .sum::<i32>()
    }

    fn new() -> Board {
        Board {
            board: [(0, false); BOARD_SIZE],
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).unwrap();

    let numbers = numbers
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = vec![];

    loop {
        let mut buf = String::new();
        let blank = io::stdin().read_line(&mut buf)?;
        if blank == 0 {
            break;
        }

        let mut board = Board::new();
        for r in 0..5 {
            let mut row = String::new();
            io::stdin().read_line(&mut row)?;
            let row = row
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            for (c, val) in row.into_iter().enumerate() {
                board[(r, c)] = (val, false);
            }
        }

        boards.push(board);
    }

    let now = time::Instant::now();

    let mut finished = vec![(0, 0); boards.len()];

    for (i, mut b) in boards.into_iter().enumerate() {
        for (j, &n) in numbers.iter().enumerate() {
            b.mark(n);
            if b.check() {
                finished[i] = (j, b.score(n));
                break;
            }
        }
    }

    let winning_score = finished
        .iter()
        .filter(|(turns, _)| turns >= &4)
        .min_by(|(turns_a, _), (turns_b, _)| turns_a.partial_cmp(turns_b).unwrap())
        .map(|(_, score)| score)
        .unwrap();

    let losing_score = finished
        .iter()
        .filter(|(turns, _)| turns >= &4)
        .max_by(|(turns_a, _), (turns_b, _)| turns_a.partial_cmp(turns_b).unwrap())
        .map(|(_, score)| score)
        .unwrap();

    let elapsed = now.elapsed();

    println!("winning score: {}", winning_score);
    println!("losing score: {}", losing_score);
    println!("calculation took: {}µs", elapsed.as_micros());

    Ok(())
}
