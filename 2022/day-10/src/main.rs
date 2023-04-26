fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let mut x = 1;
    let mut cycle = 0i32;
    let mut sum = 0;

    let mut screen = [' '; 240];

    for line in input.lines() {
        if let -1..=1 = (cycle % 40) - x {
            screen[cycle as usize] = '█';
        }

        if (cycle - 19) % 40 == 0 {
            sum += (cycle + 1) * x;
        }

        cycle += 1;

        let inst = line.split_whitespace().collect::<Vec<_>>();

        if inst[0] != "noop" {
            let value = inst[1].parse::<i32>().unwrap();
            if (cycle - 19) % 40 == 0 {
                sum += (cycle + 1) * x;
            }

            if let -1..=1 = (cycle % 40) - x {
                screen[cycle as usize] = '█';
            }

            cycle += 1;
            x += value;
        }
    }

    println!("{sum}");

    for scan in screen.chunks(40) {
        println!("{}", scan.iter().copied().collect::<String>());
    }
}
