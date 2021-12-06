use aoc::{fast_exp_mod, Col9, Mat9};
use num_bigint::BigUint;
use std::{convert::TryInto, io, time};

fn main() {
    // Read in fish
    let big_prime: BigUint = BigUint::from(18446744073709551557u64);

    let mut fish = String::new();
    let _ = io::stdin().read_line(&mut fish);

    let fish = fish
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let fish: [usize; 9] = (0..9)
        .map(|i| fish.iter().filter(|&&d| d == i).count())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let fish = Col9::from(fish);

    let now = time::Instant::now();

    // multiplying mat * fish gives a column vector for number of fish
    // the next day
    let mat: [[usize; 9]; 9] = [
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let mat = Mat9::from(mat);

    // Calculate the matrix for 256 days
    let mat = fast_exp_mod(mat.clone(), 80, &big_prime);

    let res: BigUint = mat.col_mul_mod(&fish, &big_prime).iter_column().sum::<BigUint>() % big_prime;

    let elapsed = now.elapsed();

    println!("took: {}µs", elapsed.as_micros());
    println!("{}", res);
}
