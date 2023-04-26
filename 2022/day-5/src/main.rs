#![feature(iter_array_chunks)]

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let mut input = input.split("\n\n");

    let mut initial_state = input.next().unwrap().lines().rev();
    let procedure = input.next().unwrap();

    let stack_count = initial_state.next().unwrap().split_whitespace().count();

    let mut stacks = vec![vec![]; stack_count];
    initial_state.for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push(c));
    });

    let (mut stacks1, mut stacks2) = (stacks.clone(), stacks);

    procedure.lines().for_each(|line| {
        let command = line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok());
        let [n, from, to] = command.array_chunks().next().unwrap();
        let len = stacks1[from - 1].len();

        let v1 = stacks1[from - 1].split_off(len - n).into_iter().rev();
        stacks1[to - 1].extend(v1);

        let mut v2 = stacks2[from - 1].split_off(len - n);
        stacks2[to - 1].append(&mut v2);
    });

    let part1 = stacks1.iter().filter_map(|v| v.last()).collect::<String>();
    println!("part 1: {part1}");

    let part2 = stacks2.iter().filter_map(|v| v.last()).collect::<String>();
    println!("part 2: {part2}");
}
