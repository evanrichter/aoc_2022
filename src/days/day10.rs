use std::str::FromStr;

pub fn solve(input: &str) -> String {
    let input = input.trim();
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

fn part1(input: &str) -> &str {
    input
}

fn part2(input: &str) -> usize {
    0
}

struct Cpu {
    ticks: usize,
    x: i16,
}

impl Cpu {
    fn tick(&mut self, )
}

enum Instruction {
    Nop,
    AddX { immediate: i16 },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Nop)
        } else if let Some((_, val)) = s.split_once("addx ") {
            let immediate = val.parse().map_err(drop)?;
            Ok(Self::AddX { immediate })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            "",
            part1(
                "noop\n\
                addx 3\n\
                addx -5"
            )
        );
        assert_eq!(
            36,
            part2(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )
        );
    }
}
