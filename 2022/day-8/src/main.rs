fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let grid = input
        .lines()
        .map(|s| {
            s.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut visible = vec![vec![false; grid[0].len()]; grid.len()];

    for y in 0..grid.len() {
        grid[y].iter().enumerate().fold(-1, |tallest, (i, h)| {
            visible[y][i] |= *h > tallest;
            std::cmp::max(tallest, *h)
        });

        grid[y]
            .iter()
            .enumerate()
            .rev()
            .fold(-1, |tallest, (i, h)| {
                visible[y][i] |= *h > tallest;
                std::cmp::max(tallest, *h)
            });
    }

    for x in 0..grid[0].len() {
        let mut tallest_tb = -1;
        for y in 0..grid.len() {
            if grid[y][x] > tallest_tb {
                tallest_tb = grid[y][x];
                visible[y][x] = true;
            }
        }

        let mut tallest_bt = -1;
        for y in (0..grid.len()).rev() {
            if grid[y][x] > tallest_bt {
                tallest_bt = grid[y][x];
                visible[y][x] = true;
            }
        }
    }

    println!(
        "{}",
        visible
            .iter()
            .map(|v| v.iter().filter(|b| **b).count())
            .sum::<usize>()
    );

    let mut scenic = vec![vec![1; grid[0].len()]; grid.len()];

    for y in 0..grid.len() {
        // looking left
        let mut last_seen = [0; 10];
        for (i, x) in (0..grid[y].len()).enumerate() {
            let height = grid[y][x];
            let last = last_seen[height as usize..].iter().max().unwrap();
            scenic[y][x] *= i as i32 - last;
            last_seen[height as usize] = i as i32;
        }

        // looking right
        let mut last_seen = [0; 10];
        for (i, x) in (0..grid[y].len()).rev().enumerate() {
            let height = grid[y][x];
            let last = last_seen[height as usize..].iter().max().unwrap();
            scenic[y][x] *= i as i32 - last;
            last_seen[height as usize] = i as i32;
        }
    }

    for x in 0..grid[0].len() {
        // looking up
        let mut last_seen = [0; 10];
        for (i, y) in (0..grid.len()).enumerate() {
            let height = grid[y][x];
            let last = last_seen[height as usize..].iter().max().unwrap();
            scenic[y][x] *= i as i32 - last;
            last_seen[height as usize] = i as i32;
        }

        // looking down
        let mut last_seen = [0; 10];
        for (i, y) in (0..grid.len()).rev().enumerate() {
            let height = grid[y][x];
            let last = last_seen[height as usize..].iter().max().unwrap();
            scenic[y][x] *= i as i32 - last;
            last_seen[height as usize] = i as i32;
        }
    }

    println!(
        "{}",
        scenic.iter().filter_map(|v| v.iter().max()).max().unwrap()
    );
}
