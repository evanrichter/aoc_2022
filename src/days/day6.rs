pub fn solve(input: String) -> String {
    let input = input.trim();
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

fn part1(input: &str) -> usize {
    find_preamble(input, 4)
}

fn part2(input: &str) -> usize {
    find_preamble(input, 14)
}

fn find_preamble(input: &str, window_size: usize) -> usize {
    for (index, window) in input.as_bytes().windows(window_size).enumerate() {
        // first valid window starts after window_size chars are already received
        // this window index is 0, but the char index would be window_size
        let index = index + window_size;

        let set: std::collections::HashSet<&u8> = window.iter().collect();
        if set.len() == window_size {
            return index;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    }
}
