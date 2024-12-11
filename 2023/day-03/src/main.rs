use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

fn main() {
    let lines = stdin()
        .lines()
        .map_while(Result::ok)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    // Tracks indices that touch symbols and which symbols they touch.
    // There's an unhandled edge case here, but... fuck it
    let mut symbols = HashMap::new();
    // Tracks possible gears and their adjacent numbers
    let mut gears = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        let row = row as isize;
        for (col, c) in line.chars().enumerate() {
            let col = col as isize;

            // Track the possible gears
            if c == '*' {
                gears.insert((row, col), vec![]);
            }

            if !c.is_ascii_digit() && c != '.' {
                symbols.insert((row - 1, col - 1), (row, col));
                symbols.insert((row - 1, col), (row, col));
                symbols.insert((row - 1, col + 1), (row, col));
                symbols.insert((row, col - 1), (row, col));
                symbols.insert((row, col + 1), (row, col));
                symbols.insert((row + 1, col - 1), (row, col));
                symbols.insert((row + 1, col), (row, col));
                symbols.insert((row + 1, col + 1), (row, col));
            }
        }
    }

    let mut sum = 0;

    for (row, line) in lines.iter().enumerate() {
        let row = row as isize;
        let mut val = 0;
        let mut is_included = false;
        let mut maybe_gears = HashSet::new();
        for (col, c) in line.chars().enumerate() {
            let col = col as isize;
            if let Some(digit) = c.to_digit(10) {
                val = val * 10 + digit;
                if let Some(&(gear_row, gear_col)) = symbols.get(&(row, col)) {
                    maybe_gears.insert((gear_row, gear_col));
                    is_included = true;
                }
            } else {
                if is_included {
                    sum += val;
                }
                for (gear_row, gear_col) in maybe_gears.drain() {
                    if let Some(ratios) = gears.get_mut(&(gear_row, gear_col)) {
                        ratios.push(val);
                    }
                }
                maybe_gears.clear();
                val = 0;
                is_included = false;
            }
        }
        if is_included {
            sum += val;
        }

        for (gear_row, gear_col) in maybe_gears.drain() {
            if let Some(ratios) = gears.get_mut(&(gear_row, gear_col)) {
                ratios.push(val);
            }
        }
    }

    let mut ratio_sum = 0;

    for ratios in gears.values().filter(|v| v.len() == 2) {
        ratio_sum += ratios[0] * ratios[1];
    }

    println!("{sum}");
    println!("{ratio_sum}");
}
