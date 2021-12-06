use std::convert::TryInto;

use num_bigint::BigUint;

#[allow(unused)]
pub struct Col9 {
    data: [BigUint; 9],
}

pub struct ColColumnIterator<'a> {
    column: &'a Col9,
    row: usize,
}

impl<'a> Iterator for ColColumnIterator<'a> {
    type Item = &'a BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.column.data.len() {
            unsafe {
                let ret = Some(self.column.data.get_unchecked(self.row));
                self.row += 1;
                ret
            }
        } else {
            None
        }
    }
}

#[allow(unused)]
impl Col9 {
    fn new() -> Col9 {
        Col9 {
            data: [0u64; 9]
                .iter()
                .map(|&i| BigUint::from(i))
                .collect::<Vec<BigUint>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn from(data: [usize; 9]) -> Col9 {
        Col9 {
            data: data
                .iter()
                .map(|&i| BigUint::from(i))
                .collect::<Vec<BigUint>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn iter_column<'a>(&'a self) -> ColColumnIterator<'a> {
        ColColumnIterator {
            column: &self,
            row: 0,
        }
    }
}

#[derive(Clone)]
pub struct Mat9 {
    data: [[BigUint; 9]; 9],
}

impl Mat9 {
    fn new() -> Mat9 {
        Mat9 {
            data: [[0u64; 9]; 9]
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|&i| BigUint::from(i))
                        .collect::<Vec<BigUint>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[BigUint; 9]>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn from(data: [[usize; 9]; 9]) -> Mat9 {
        Mat9 {
            data: data
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|&i| BigUint::from(i))
                        .collect::<Vec<BigUint>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[BigUint; 9]>>()
                .try_into()
                .unwrap(),
        }
    }

    fn mat_mul(&self, rhs: &Mat9) -> Mat9 {
        let mut ret = Mat9::new();

        for r in 0..9 {
            for c in 0..9 {
                ret.data[r][c] = self
                    .iter_row(r)
                    .zip(rhs.iter_column(c))
                    .map(|(u, v)| u * v)
                    .sum();
            }
        }

        ret
    }

    fn mat_mul_mod(&self, rhs: &Mat9, modr: &BigUint) -> Mat9 {
        let mut ret = Mat9::new();

        for r in 0..9 {
            for c in 0..9 {
                ret.data[r][c] = self
                    .iter_row(r)
                    .zip(rhs.iter_column(c))
                    .map(|(u, v)| (u * v) % modr)
                    .sum();
            }
        }

        ret
    }

    pub fn col_mul(&self, rhs: &Col9) -> Col9 {
        let mut ret = Col9::new();
        for r in 0..9 {
            ret.data[r] = self
                .iter_row(r)
                .zip(rhs.iter_column())
                .map(|(u, v)| u * v)
                .sum();
        }
        ret
    }

    pub fn col_mul_mod(&self, rhs: &Col9, modr: &BigUint) -> Col9 {
        let mut ret = Col9::new();
        for r in 0..9 {
            ret.data[r] = self
                .iter_row(r)
                .zip(rhs.iter_column())
                .map(|(u, v)| (u * v) % modr)
                .sum();
        }
        ret
    }
}

pub struct MatRowIterator<'a> {
    mat: &'a Mat9,
    col: usize,
    row: usize,
}

impl<'a> Iterator for MatRowIterator<'a> {
    type Item = &'a BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.mat.data.len() && self.col < self.mat.data[self.row].len() {
            unsafe {
                let ret = Some(
                    self.mat
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

pub struct MatRowsIterator<'a> {
    mat: &'a Mat9,
    row: usize,
}

impl<'a> Iterator for MatRowsIterator<'a> {
    type Item = MatRowIterator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.mat.data.len() {
            let ret = Some(MatRowIterator {
                mat: self.mat,
                row: self.row,
                col: 0,
            });
            self.row += 1;
            ret
        } else {
            None
        }
    }
}

pub struct MatColumnIterator<'a> {
    mat: &'a Mat9,
    col: usize,
    row: usize,
}

impl<'a> Iterator for MatColumnIterator<'a> {
    type Item = &'a BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.mat.data.len() && self.col < self.mat.data[self.row].len() {
            unsafe {
                let ret = Some(
                    self.mat
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

pub struct MatColumnsIterator<'a> {
    mat: &'a Mat9,
    col: usize,
}

impl<'a> Iterator for MatColumnsIterator<'a> {
    type Item = MatColumnIterator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.mat.data[0].len() {
            let ret = Some(MatColumnIterator {
                mat: self.mat,
                row: 0,
                col: self.col,
            });
            self.col += 1;
            ret
        } else {
            None
        }
    }
}

impl Mat9 {
    pub fn iter_rows<'a>(&'a self) -> MatRowsIterator<'a> {
        MatRowsIterator { mat: &self, row: 0 }
    }

    fn iter_row<'a>(&'a self, row: usize) -> MatRowIterator<'a> {
        MatRowIterator {
            mat: &self,
            col: 0,
            row,
        }
    }

    pub fn iter_columns<'a>(&'a self) -> MatColumnsIterator<'a> {
        MatColumnsIterator { mat: &self, col: 0 }
    }

    fn iter_column<'a>(&'a self, col: usize) -> MatColumnIterator<'a> {
        MatColumnIterator {
            mat: &self,
            col,
            row: 0,
        }
    }
}

pub fn fast_exp(m: Mat9, e: usize) -> Mat9 {
    if e < 1 {
        panic!("Exponent can not be < 1!");
    }

    let highest_bit = (0..64)
        .filter(|i| ((e >> i) & 1) == 1)
        .max()
        .unwrap()
        + 1;

    let mut products = Vec::with_capacity(64);

    products.push(m);
    (1..highest_bit).for_each(|i| {
        products.push(products[i - 1].mat_mul(&products[i - 1]));
    });

    let mut products = (0..highest_bit)
        .filter(|i| ((e >> i) & 1) == 1)
        .map(|i| &products[i]);

    let mut base = products.next().unwrap().clone();

    while let Some(m) = products.next() {
        base = base.mat_mul(m);
    }

    base
}

pub fn fast_exp_mod(m: Mat9, e: usize, modr: &BigUint) -> Mat9 {
    if e < 1 {
        panic!("Exponent can not be < 1!");
    }

    let highest_bit = (0..64)
        .filter(|i| ((e >> i) & 1) == 1)
        .max()
        .unwrap()
        + 1;

    let mut products = Vec::with_capacity(64);

    products.push(m);
    (1..highest_bit).for_each(|i| {
        products.push(products[i - 1].mat_mul_mod(&products[i - 1], modr));
    });

    let mut products = (0..highest_bit)
        .filter(|i| ((e >> i) & 1) == 1)
        .map(|i| &products[i]);

    let mut base = products.next().unwrap().clone();

    while let Some(m) = products.next() {
        base = base.mat_mul_mod(m, modr);
    }

    base
}
