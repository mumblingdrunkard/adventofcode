use day_10::solve;

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    println!("{:?}", solve(lines.into_iter()));
}
