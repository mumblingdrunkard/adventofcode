// # Move encoding:
//
// Rock:     0
// Paper:    1
// Scissors: 2
//
// # Outcome encoding:
//
// Loss: -1
// Draw:  0
// Win:   1
//
// # Scoring moves:
//
// Move + 1
//
// # Scoring outcomes
//
// (Outcome + 1) * 3
//
// E.g.: 2 * 3 = 6 for win
//
// # Scoring the game
//
// Move score + Outcome score

/// Calculates the score given ascii encoded (opponent move, your move)
fn score1(game: (u8, u8)) -> i32 {
    // # Calculating outcome from given moves
    //
    // (You - Opponent + 4) % 3 - 1
    //
    // E.g.: You: Rock = 0, Opponent: Paper = 1
    // (0 - 1 + 4) % 3 - 1 = -1 = Loss

    // map opponent move to a number 0..=2
    let opponent = (game.0 - b'A') as i32;

    // map your move to a number 0..=2
    let you = (game.1 - b'X') as i32;

    // calculate the outcome as a number -1..=1
    let outcome = (you - opponent + 4) % 3 - 1;

    1 + you + (outcome + 1) * 3
}

/// Calculates the score given ascii encoded (opponent move, desired outcome)
fn score2(game: (u8, u8)) -> i32 {
    // # Calculating move from opponent move and desired outcome
    //
    // (opponent + outcome + 3) % 3
    //
    // E.g.: Opponent: Scissors = 2, Outcome: Loss = -1
    // (2 - 1 + 3) % 3 = 1 = Paper

    // mop opponent move to a number 0..=2
    let opponent = (game.0 - b'A') as i32;

    // map outcome to a number -1..=1
    let outcome = (game.1 - b'X') as i32 - 1;

    // calculate the move made by you to ensure the outcome
    let you = (opponent + outcome + 3) % 3;

    1 + you + (outcome + 1) * 3
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let score1: i32 = input
        .lines()
        .filter_map(|line| line.split_once(' ')) // filter map skips dealing with None cases
        .map(|(first, second)| (first.as_bytes()[0], second.as_bytes()[0]))
        .map(score1)
        .sum();

    println!("part 1: {score1}");

    let score2: i32 = input
        .lines()
        .filter_map(|line| line.split_once(' ')) // filter map skips dealing with None cases
        .map(|(first, second)| (first.as_bytes()[0], second.as_bytes()[0]))
        .map(score2)
        .sum();

    println!("part 2: {score2}");
}
