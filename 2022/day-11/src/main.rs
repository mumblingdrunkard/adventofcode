mod monkey;

use monkey::Monkey;
use std::{collections::BTreeSet, str::FromStr};

fn main() -> Result<(), ()> {
    let t0 = std::time::Instant::now();

    let input = std::fs::read_to_string("./input").map_err(|_| ())?;

    let monkeys = input
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>, ()>>()?;

    let t1 = std::time::Instant::now();

    // part 1
    let mut my_monkeys = monkeys.clone();

    let mut inspected = vec![0; my_monkeys.len()];

    for _ in 0..20 {
        for i in 0..my_monkeys.len() {
            let mut targets = vec![];
            let monkey = &mut my_monkeys[i];
            inspected[i] += monkey.items.len();
            for item in monkey.items.drain(..) {
                let item = monkey.operation.apply(item);
                let target = monkey.test.apply(item);
                targets.push((target, item));
            }

            for (target, item) in targets {
                my_monkeys[target].items.push(item);
            }
        }
    }

    let part1: usize = inspected
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .rev()
        .take(2)
        .product();

    let t2 = std::time::Instant::now();

    // part 2
    let mut my_monkeys = monkeys;

    let mut inspected = vec![0; my_monkeys.len()];

    // modulo under the product of all the tests does not impact the result of the tests
    let z: i64 = my_monkeys
        .iter()
        .map(|monkey| monkey.test.divisor)
        .product();

    for _ in 0..10000 {
        for i in 0..my_monkeys.len() {
            let mut targets = vec![];
            let monkey = &mut my_monkeys[i];
            inspected[i] += monkey.items.len();
            for item in monkey.items.drain(..) {
                let item = monkey.operation.apply_z(item, z);
                let target = monkey.test.apply(item);
                targets.push((target, item));
            }

            for (target, item) in targets {
                my_monkeys[target].items.push(item);
            }
        }
    }

    let part2: usize = inspected
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .rev()
        .take(2)
        .product();

    let t3 = std::time::Instant::now();

    println!("part 1: {part1}");
    println!("part 2: {part2}");

    println!("Reading and parsing took {:?}", t1 - t0);
    println!("Part 1 took {:?}", t2 - t1);
    println!("Part 2 took {:?}", t3 - t2);

    Ok(())
}
