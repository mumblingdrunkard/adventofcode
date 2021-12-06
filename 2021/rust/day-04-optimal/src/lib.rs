const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;

type BoardData = [[(usize, i32); BOARD_WIDTH]; BOARD_HEIGHT];

pub struct Board {
    data: BoardData,
}

/// Iterator over a single column of a `Board`
pub struct ColumnIterator<'a> {
    col: usize,
    row: usize,
    board: &'a Board,
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = &'a (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.board.data.len() && self.col < self.board.data[self.row].len() {
            unsafe {
                let ret = Some(
                    self.board
                        .data
                        .get_unchecked(self.row)
                        .get_unchecked(self.col),
                );
                self.row += 1;
                ret
            }
        } else {
            None
        }
    }
}

/// Iterator over `ColumnIterator`s for all columns of a `Board`
pub struct BoardColumnIterator<'a> {
    col: usize,
    board: &'a Board,
}

impl<'a> Iterator for BoardColumnIterator<'a> {
    type Item = ColumnIterator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.board.data[0].len() {
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

/// Iterator over a single row of a `Board`
pub struct RowIterator<'a> {
    col: usize,
    row: usize,
    board: &'a Board,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = &'a (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.board.data[self.row].len() {
            unsafe {
                let ret = Some(
                    self.board
                        .data
                        .get_unchecked(self.row)
                        .get_unchecked(self.col),
                );
                self.col += 1;
                ret
            }
        } else {
            None
        }
    }
}

/// Iterator over `RowIterator`s for all rows of a `Board`
pub struct BoardRowIterator<'a> {
    row: usize,
    board: &'a Board,
}

impl<'a> Iterator for BoardRowIterator<'a> {
    type Item = RowIterator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.board.data.len() {
            let ret = Some(RowIterator {
                col: 0,
                row: self.row,
                board: self.board,
            });
            self.row += 1;
            ret
        } else {
            None
        }
    }
}

impl Board {
    /// Creates a new, empty `Board`
    pub fn new() -> Board {
        Board {
            data: [[(0, 0); BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    /// Creates a board from a slice
    ///
    /// E.g. for board of width and height 2
    /// [1, 2, 3, 4] -> [[1, 2], [3, 4]]
    pub fn from_slice(data: &[(usize, i32)]) -> Board {
        let mut board = Board::new();

        board
            .data
            .iter_mut()
            .flat_map(|r| r.iter_mut())
            .zip(data)
            .for_each(|(a, b)| {
                a.0 = b.0;
                a.1 = b.1
            });

        board
    }

    /// Returns an iterator over the columns of the board
    pub fn iter_col<'a>(&'a self) -> BoardColumnIterator<'a> {
        BoardColumnIterator {
            col: 0,
            board: &self,
        }
    }

    /// Returns an iterator over the rows of the board
    pub fn iter_row<'a>(&'a self) -> BoardRowIterator<'a> {
        BoardRowIterator {
            row: 0,
            board: &self,
        }
    }

    /// Calculates the winning round and the winning number of the board
    fn win_and_winning_number(&self) -> (usize, i32) {
        let row_wise = self
            .iter_row()
            .map(|r| {
                r.max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
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

    /// Calculates the winning round and the final score of the board
    pub fn win_and_score(&self) -> (usize, i32) {
        let (win, winning_number) = self.win_and_winning_number();

        (
            win,
            self.data
                .iter()
                .flat_map(|r| r.iter())
                .filter(|(round, _)| round > &win) // remove marked cells
                .map(|(_, n)| n * winning_number)
                .sum(),
        )
    }
}
