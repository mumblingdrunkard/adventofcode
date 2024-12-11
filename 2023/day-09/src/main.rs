use day_09::solve;

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();
    let (part1, part2) = solve(lines.into_iter());
    println!("{part1}");
    println!("{part2}");
}
