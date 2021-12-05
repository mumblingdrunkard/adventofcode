use std::{io::stdin, time::Instant};

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

impl Board {
    fn check(&self) -> bool {
        let row_wise = (0..BOARD_HEIGHT) // for each row
            .map(|r| {
                (0..BOARD_WIDTH) // all entries are marked
                    .map(|c| self[(r, c)].1)
                    .all(|v| v == true)
            })
            .any(|v| v == true); // any row exists in which all entries are marked

        let column_wise = (0..BOARD_WIDTH) // for each column
            .map(|c| {
                (0..BOARD_HEIGHT) // all entries are marked
                    .map(|r| self[(r, c)].1)
                    .all(|v| v == true)
            })
            .any(|v| v == true); // any column exists in which all entries are marked

        column_wise || row_wise
    }

    fn mark(&mut self, n: i32) {
        for mut s in &mut self.board {
            if s.0 == n {
                s.1 = true;
            }
        }
    }

    fn score(&self, n: i32) -> i32 {
        n * self
            .board
            .iter()
            .map(|v| match v.1 {
                false => v.0,
                true => 0, // exclude marked values from the calculation
            })
            .sum::<i32>()
    }
}

fn main() -> std::io::Result<()> {
    let mut numbers = String::new();
    stdin().read_line(&mut numbers).unwrap();

    let numbers = numbers
        .trim() // cuts off \n from read_line
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = vec![];

    loop {
        let mut buf = String::new();
        let blank = stdin().read_line(&mut buf)?;
        if blank == 0 {
            break;
        }

        let mut data = [0; 25];
        for r in 0..5 {
            let mut row = String::new();
            stdin().read_line(&mut row)?;
            let row = row
                .trim() // cuts off \n from read_line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            for (c, val) in row.into_iter().enumerate() {
                data[r * BOARD_WIDTH + c] = val;
            }
        }

        let board = Board {
            board: data.map(|v| (v, false)),
        };

        boards.push(board);
    }

    let now = Instant::now();

    let mut finished = vec![(0, 0); boards.len()];

    // For each board
    for (i, mut b) in boards.into_iter().enumerate() {
        // Mark numbers until the board wins
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
        .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap()) // find least amount of turns
        .map(|t| t.1) // get score
        .unwrap();

    let losing_score = finished
        .iter()
        .filter(|(turns, _)| turns >= &4)
        .max_by(|(turns_a, _), (turns_b, _)| turns_a.partial_cmp(turns_b).unwrap()) // find most amount of turns
        .map(|(_, score)| score) // get score
        .unwrap();

    let elapsed = now.elapsed();

    println!("winning score: {}", winning_score);
    println!("losing score: {}", losing_score);
    println!("calculation took: {}µs", elapsed.as_micros());

    Ok(())
}
