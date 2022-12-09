pub fn solve(input: &str) -> String {
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("{part1}, {part2}")
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    // determine number of stacks
    let (crate_str, num_line) = input.split_once('1').unwrap();
    let num_stacks: usize = num_line
        .lines() // just get one line
        .next()
        .unwrap()
        .split_whitespace() // split to get row names
        .last()
        .unwrap()
        .parse() // parse the last row number to get length
        .unwrap();
    let mut stacks = vec![Vec::new(); num_stacks];

    // put crates in their stacks
    for crate_line in crate_str.lines().rev().skip(1) {
        for (stack, c) in crate_line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[stack].push(c);
            }
        }
    }

    stacks
}

struct Move {
    count: usize,
    source: usize,
    dest: usize,
}

fn parse_commands(input: &str) -> Vec<Move> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (_, l) = l.split_once("move ").unwrap();
            let (count, l) = l.split_once(" from ").unwrap();
            let (source, dest) = l.split_once(" to ").unwrap();
            Move {
                count: count.parse().unwrap(),
                source: source.parse::<usize>().unwrap() - 1,
                dest: dest.parse::<usize>().unwrap() - 1,
            }
        })
        .collect()
}

fn part1(input: &str) -> String {
    let (init_state, commands) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(init_state);
    let commands = parse_commands(commands);

    for mov in commands {
        for _ in 0..mov.count {
            let krate = stacks[mov.source].pop().unwrap();
            stacks[mov.dest].push(krate);
        }
    }

    // get top crates
    stacks.iter().map(|v| v.last().unwrap()).collect()
}

fn part2(input: &str) -> String {
    let (init_state, commands) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(init_state);
    let commands = parse_commands(commands);

    for mov in commands {
        // get the source and dest vectors out of stacks (prevents multiple &mut)
        let mut source = Vec::new();
        let mut dest = Vec::new();
        std::mem::swap(&mut stacks[mov.source], &mut source);
        std::mem::swap(&mut stacks[mov.dest], &mut dest);

        let krates = &source[source.len() - mov.count..];
        // put crates in new place
        dest.extend_from_slice(krates);
        // remove crates from old
        source.truncate(source.len() - mov.count);

        // put src, dest back in stacks
        std::mem::swap(&mut stacks[mov.source], &mut source);
        std::mem::swap(&mut stacks[mov.dest], &mut dest);
    }

    // get top crates
    stacks.iter().map(|v| v.last().unwrap()).collect()
}
