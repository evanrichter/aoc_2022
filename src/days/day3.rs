use std::collections::HashSet;

pub fn solve(input: String) -> String {
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("{part1}, {part2}")
}

fn part1(input: &str) -> String {
    let sum: u64 = input
        .lines()
        .map(split)
        .map(|(left, right)| common(left, right))
        .map(|c| priority(c) as u64)
        .sum();

    format!("{sum}")
}

fn split(input: &str) -> (&str, &str) {
    let len = input.len() / 2;
    (&input[0..len], &input[len..])
}

fn common(a: &str, b: &str) -> char {
    let a: HashSet<char> = a.chars().collect();
    let b: HashSet<char> = b.chars().collect();

    let common: HashSet<char> = &a & &b;
    common.into_iter().next().unwrap()
}

fn priority(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        (c as u8) - b'a' + 1
    } else {
        (c as u8) - b'A' + 27
    }
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let mut badges = Vec::new();

    // loop one group at a time (3 lines)
    loop {
        let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) else {
            break;
        };
        let badge = common3(a, b, c);
        badges.push(badge);
    }

    let sum: u64 = badges.iter().map(|&b| priority(b) as u64).sum();
    format!("{sum}")
}

fn common3(a: &str, b: &str, c: &str) -> char {
    let a: HashSet<char> = a.chars().collect();
    let b: HashSet<char> = b.chars().collect();
    let c: HashSet<char> = c.chars().collect();

    let common: HashSet<char> = &(&a & &b) & &c;
    common.into_iter().next().unwrap()
}
