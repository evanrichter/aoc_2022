pub fn solve(input: String) -> String {
    // part 1
    let calorie_stream: Vec<Option<u32>> = input.lines().map(|s| s.parse().ok()).collect();
    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut elf = Vec::new();
    for c in calorie_stream {
        match c {
            Some(val) => elf.push(val),
            None => {
                // elf is done
                elves.push(elf.clone());
                elf.clear();
            }
        }
    }

    // find biggest elf
    let part1 = elves
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .max()
        .expect("at least one elf");

    // part 2

    // collect the sums, then sort
    let mut sums: Vec<u32> = elves.iter().map(|v| v.iter().sum::<u32>()).collect();
    sums.sort_unstable();
    sums.reverse();

    // get the top 3
    let &[a, b, c, ..] = sums.as_slice() else {
            panic!("not even 3 elves?");
        };
    let part2 = a + b + c;

    format!("{part1}, {part2}")
}
