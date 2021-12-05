use std::collections::HashMap;
use std::{io, time};

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

struct Board {
    board: [(usize, i32); BOARD_SIZE],
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = (usize, i32);

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
    fn new() -> Board {
        Board {
            board: [(0, 0); BOARD_SIZE],
        }
    }

    // O(BOARD_SIZE) = O(1)
    fn win_and_winning_number(&self) -> (usize, i32) {
        let row_wise = (0..BOARD_HEIGHT)
            .map(|r| {
                (0..BOARD_WIDTH)
                    .map(|c| self[(r, c)])
                    .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
                    .unwrap()
            })
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
            .unwrap();

        let column_wise = (0..BOARD_WIDTH)
            .map(|c| {
                (0..BOARD_HEIGHT)
                    .map(|r| self[(r, c)])
                    .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
                    .unwrap()
            })
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
            .unwrap();

        if row_wise.0 < column_wise.0 {
            row_wise
        } else {
            column_wise
        }
    }

    // O(1)
    fn win_and_score(&self) -> (usize, i32) {
        let (win, winning_number) = self.win_and_winning_number();

        (
            win,
            self.board
                .iter()
                .filter(|(round, _)| round > &win) // remove marked cells
                .map(|(_, n)| n * winning_number)
                .sum(),
        )
    }
}

// O(b + n)
fn main() -> std::io::Result<()> {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).unwrap();

    // O(n)
    let numbers = numbers
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // O(n)
    let drawn = numbers
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect::<HashMap<i32, usize>>();

    let mut boards = vec![];

    // O(b)
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
                board[(r, c)] = (drawn[&val], val); // O(1) lookup
            }
        }

        boards.push(board);
    }

    let now = time::Instant::now();

    // O(b × 1) = O(b)
    let winner = boards
        .iter()
        .map(|b| b.win_and_score())
        .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
        .map(|(_, score)| score)
        .unwrap();

    // O(b × 1) = O(b)
    let loser = boards
        .iter()
        .map(|b| b.win_and_score())
        .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
        .map(|(_, score)| score)
        .unwrap();

    let elapsed = now.elapsed();

    println!("calculation took: {}µs", elapsed.as_micros());

    println!("winner: {}", winner);
    println!("loser: {}", loser);

    Ok(())
}
