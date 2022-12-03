pub fn solve(input: String) -> String {
    // part 1
    let part1: u32 = input
        .split("\n")
        .map(|l| {
            if l.is_empty() {
                0
            } else {
                let (them, me) = l.split_once(" ").unwrap();
                let shape_score = match me {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => panic!(),
                };
                let outcome_score = score(them.chars().next().unwrap(), me.chars().next().unwrap());
                shape_score + outcome_score
            }
        })
        .sum();

    // part 2
    let part2: u32 = input
        .split("\n")
        .map(|l| {
            if l.is_empty() {
                0
            } else {
                let (them, me) = l.split_once(" ").unwrap();
                let outcome_score = match me {
                    "X" => 0,
                    "Y" => 3,
                    "Z" => 6,
                    _ => panic!(),
                };
                let shape = play_needed(them.chars().next().unwrap(), outcome_score);
                let shape_score = match shape {
                    'X' => 1,
                    'Y' => 2,
                    'Z' => 3,
                    _ => panic!(),
                };
                shape_score + outcome_score
            }
        })
        .sum();

    format!("{part1}, {part2}")
}

fn score(them: char, me: char) -> u32 {
    let them = them as u8 - 'A' as u8;
    let me = me as u8 - 'X' as u8;

    match (them, me) {
        // ties
        (0, 0) => 3,
        (1, 1) => 3,
        (2, 2) => 3,
        // wins
        (0, 1) => 6,
        (1, 2) => 6,
        (2, 0) => 6,
        // losses
        (1, 0) => 0,
        (2, 1) => 0,
        (0, 2) => 0,
        // all others are wat
        _ => panic!(),
    }
}

fn play_needed(them: char, score_needed: u32) -> char {
    for play in ['X', 'Y', 'Z'] {
        if score(them, play) == score_needed {
            return play;
        }
    }
    panic!()
}
