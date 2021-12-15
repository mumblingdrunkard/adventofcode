use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: i32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> io::Result<()> {
    let mut map = vec![];
    let mut costs = vec![];

    loop {
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line)?;
        if n == 0 {
            break;
        }

        let line = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();

        map.push(line);
    }

    for _ in 0..map.len() * 5 {
        costs.push(
            (0..map.len() * 5)
                .map(|_| (i32::MAX, (0, 0)))
                .collect::<Vec<_>>(),
        );
    }

    println!("{}", map.len());
    println!("{}", map[0].len());

    println!("");

    println!("{}", costs.len());
    println!("{}", costs[0].len());

    let mut pqueue = BinaryHeap::new();

    pqueue.push(State {
        cost: 0,
        position: (0, 0),
    });

    while !pqueue.is_empty() {
        // get top element
        let state = pqueue.pop().unwrap();

        let cost = state.cost;
        let (y, x) = state.position;

        // check left
        if x > 0 {
            let (cost1, (_, _)) = costs[y][x - 1];
            let div_x = ((x - 1) / map[0].len()) as i32;
            let div_y = (y / map.len()) as i32;
            let mut risk1 =
                map[y % map.len()][(x + map[0].len() - 1) % map[0].len()] + div_x + div_y;
            while risk1 > 9 {
                risk1 -= 9;
            }

            if cost + risk1 < cost1 {
                // update entry at (y, x-1)
                costs[y][x - 1] = (cost + risk1, (y, x));
                pqueue.push(State {
                    cost: cost + risk1,
                    position: (y, x - 1),
                });
            }
        }

        // check right
        if x < costs[0].len() - 1 {
            let (cost1, (_, _)) = costs[y][x + 1];
            let div_x = ((x + 1) / map[0].len()) as i32;
            let div_y = (y / map.len()) as i32;
            let mut risk1 = map[y % map.len()][(x + 1) % map[0].len()] + div_x + div_y;
            while risk1 > 9 {
                risk1 -= 9;
            }
            if cost + risk1 < cost1 {
                // update entry at (y, x-1)
                costs[y][x + 1] = (cost + risk1, (y, x));
                pqueue.push(State {
                    cost: cost + risk1,
                    position: (y, x + 1),
                });
            }
        }

        // check up
        if y > 0 {
            let (cost1, (_, _)) = costs[y - 1][x];
            let div_x = (x  / map[0].len()) as i32;
            let div_y = ((y - 1) / map.len()) as i32;
            let mut risk1 = map[(y + map.len() - 1) % map.len()][x % map[0].len()] + div_x + div_y;
            while risk1 > 9 {
                risk1 -= 9;
            }
            if cost + risk1 < cost1 {
                // update entry at (y, x-1)
                costs[y - 1][x] = (cost + risk1, (y, x));
                pqueue.push(State {
                    cost: cost + risk1,
                    position: (y - 1, x),
                });
            }
        }

        // check down
        if y < costs.len() - 1 {
            let (cost1, (_, _)) = costs[y + 1][x];
            let div_x = (x  / map[0].len()) as i32;
            let div_y = ((y + 1) / map.len()) as i32;
            let mut risk1 = map[(y + 1) % map.len()][x % map[0].len()] + div_x + div_y;
            while risk1 > 9 {
                risk1 -= 9;
            }
            if cost + risk1 < cost1 {
                // update entry at (y, x-1)
                costs[y + 1][x] = (cost + risk1, (y, x));
                pqueue.push(State {
                    cost: cost + risk1,
                    position: (y + 1, x),
                });
            }
        }
    }

    let (cost, (mut y, mut x)) = costs[costs.len() - 1][costs[0].len() - 1];

    let mut prev_cost = cost;
    while x != 0 || y != 0 {
        let (cost, (y1, x1)) = costs[y][x];
        println!("{}, {}: {}", y, x, prev_cost - cost);
        prev_cost = cost;
        y = y1;
        x = x1;
    }

    println!("{}", cost);

    Ok(())
}
