pub fn solve(input: &str) -> String {
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("{part1}, {part2}")
}

fn part1(input: &str) -> String {
    let count = input
        .trim()
        .lines()
        .map(ranges)
        .filter_map(|(a, b)| (a.fully_contains(&b) || b.fully_contains(&a)).then_some(()))
        .count();
    format!("{count}")
}

fn part2(input: &str) -> String {
    let count = input
        .trim()
        .lines()
        .map(ranges)
        .filter_map(|(a, b)| a.overlaps(&b).then_some(()))
        .count();
    format!("{count}")
}

#[derive(Clone, Copy, Debug)]
struct Range {
    beg: u32,
    end: u32,
}

impl Range {
    fn parse(s: &str) -> Range {
        let (beg, end) = s.split_once('-').unwrap();
        Range {
            beg: beg.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.beg <= other.beg && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        if self.end < other.beg {
            return false;
        }
        if other.end < self.beg {
            return false;
        }
        true
    }
}

fn ranges(line: &str) -> (Range, Range) {
    let (a, b) = line.split_once(',').unwrap();
    (Range::parse(a), Range::parse(b))
}
