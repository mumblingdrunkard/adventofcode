/// # bingo
/// B0. Convert list of numbers to HashMap `map` with (k: index, v: value)
/// B1. Read in boards and store as a tuple `(w: map[value], v: value)`
/// B2. For each board
///     B2-0. For each row, find the tuple with the highest `w`; take the smallest of these
///     B2-1. For each column, do as in B2-0.
///     B2-2. Take the smallest of these two tuples, this is the winning round and winning draw
///     B2-3. For all entries that come after the winning round, multiply with winning draw and sum
///           up, this is the score for the board.
/// B3. The winning and losing boards are the ones with the lowest and highest winning round
///     respectively.
///
/// # Steps in greater detail
///
/// ## B0.
///
/// We convert the list to a HashMap to get O(1) lookup time. We use this to get which number
/// arrives at which draw.
///
/// ## B1.
///
/// We store entries in the board as tuples of `(when number is drawn, the number)`.
///
/// ## B2.
///
/// Now, for each board, we should find: how many draws until this board wins, and score.
///
/// ### B2-0.
///
/// For each row in the board, find the entry with the highest `when number is drawn`. This will be
/// how many rounds it will take for this row to be a winning row.
/// Taking the smallest of all these, will give how many rounds it takes for at least one row to be
/// completed.
///
/// ### B2-1.
///
/// Same process as B2-0., but for columns instead, yielding the smallest number of rounds to
/// complete at least one column.
///
/// ### B2-2.
///
/// Taking the smallest of row-wise or column-wise yields the smallest number of rounds needed for
/// this board to win.
///
/// ### B2-3.
///
/// To calculate the score of the board, find all unmarked entries, multiply by the winning draw
/// and sum them all up. Unmarked entries will be all entries that come after the winning round.
///
/// ## B3.
///
/// This step is self explanatory. The board that takes the fewest turns to win is the winning
/// board and vice versa.

///
use std::collections::HashMap;
use std::{io, time};

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;

type BoardData = [[(usize, i32); BOARD_WIDTH]; BOARD_HEIGHT];

struct Board {
    board: BoardData,
}

#[derive(Clone, Copy)]
struct ColumnIterator<'a> {
    col: usize,
    row: usize,
    board: &'a BoardData,
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = &'a (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < BOARD_WIDTH && self.row < BOARD_HEIGHT {
            let ret = Some(&self.board[self.row][self.col]);
            self.row += 1;
            ret
        } else {
            None
        }
    }
}

struct BoardColumnIterator<'a> {
    col: usize,
    board: &'a BoardData,
}

impl<'a> Iterator for BoardColumnIterator<'a> {
    type Item = ColumnIterator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < BOARD_WIDTH {
            let ret = Some(ColumnIterator {
                col: self.col,
                row: 0,
                board: self.board,
            });
            self.col += 1;
            ret
        } else {
            None
        }
    }
}

impl Board {
    fn new() -> Board {
        Board {
            board: [[(0, 0); BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    fn iter_col<'a>(&'a self) -> BoardColumnIterator<'a> {
        BoardColumnIterator {
            col: 0,
            board: &self.board,
        }
    }

    // O(BOARD_SIZE) = O(1)
    fn win_and_winning_number(&self) -> (usize, i32) {
        let row_wise = self
            .board
            .iter()
            .map(|r| {
                r.iter()
                    .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
                    .unwrap()
            })
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
            .unwrap();

        let column_wise = self
            .iter_col()
            .map(|c| {
                c.max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
                    .unwrap()
            })
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
            .unwrap();

        if row_wise.0 < column_wise.0 {
            *row_wise
        } else {
            *column_wise
        }
    }

    // O(1)
    fn win_and_score(&self) -> (usize, i32) {
        let (win, winning_number) = self.win_and_winning_number();

        (
            win,
            self.board
                .iter()
                .flat_map(|r| r.iter())
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
                board.board[r][c] = (drawn[&val], val); // O(1) lookup
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

    println!("took: {}µs", elapsed.as_micros());
    println!("winner: {}", winner);
    println!("loser: {}", loser);

    Ok(())
}
