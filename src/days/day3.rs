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

fn common(left: &str, right: &str) -> char {
    let mut left = left.as_bytes().to_vec();
    let mut right = right.as_bytes().to_vec();
    left.sort_unstable();
    right.sort_unstable();

    for l in left {
        if right.contains(&l) {
            return l as char;
        }
    }
    panic!("not found")
}

fn priority(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        (c as u8) - 'a' as u8 + 1
    } else {
        (c as u8) - 'A' as u8 + 27
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
    let mut a = a.as_bytes().to_vec();
    let mut b = b.as_bytes().to_vec();
    let mut c = c.as_bytes().to_vec();
    a.sort_unstable();
    b.sort_unstable();
    c.sort_unstable();

    for a in a {
        if b.contains(&a) && c.contains(&a) {
            return a as char;
        }
    }

    panic!("not found")
}
