use nom::{
    bytes::complete::tag,
    character::complete::{i32 as i32_parser, newline},
    multi::separated_list0,
    sequence::{pair, separated_pair, tuple},
    IResult,
};
use std::collections::BTreeSet;

pub fn solve(input: &str) -> String {
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn tuning_frequency(&self) -> u64 {
        (self.x as u64) * 4_000_000 + (self.y as u64)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Sensor {
    location: Coord,
    beacon: Coord,
}

impl Sensor {
    fn coverage_region(&self, line: i32) -> Option<(i32, i32)> {
        let range = self.location.distance(&self.beacon);
        let y_diff = self.location.y.abs_diff(line);
        if y_diff > range {
            return None;
        }

        let left_x_relative = -((range - (y_diff)) as i32);
        let right_x_relative = (range - (y_diff)) as i32;

        let left = Coord {
            x: self.location.x + left_x_relative,
            y: line,
        };
        let right = Coord {
            x: self.location.x + right_x_relative,
            y: line,
        };

        Some((left.x, right.x))
    }

    fn blackout_region(&self, line: i32) -> Option<(i32, i32)> {
        // get the region covered on this line by this sensor
        let (mut left, mut right) = self.coverage_region(line)?;

        if left == right && (Coord { x: left, y: line }) == self.beacon {
            // line is at the tip of the sensor range and there is a beacon here
            return None;
        }

        if (Coord { x: left, y: line }) == self.beacon {
            // left is the beacon, so move this point right
            left += 1;
        } else if (Coord { x: right, y: line }) == self.beacon {
            // right is the beacon, so move this point left
            right -= 1;
        }

        Some((left, right))
    }
}

fn coord(i: &str) -> IResult<&str, Coord> {
    separated_pair(
        pair(tag("x="), i32_parser),
        tag(", "),
        pair(tag("y="), i32_parser),
    )(i)
    .map(|(r, ((_, x), (_, y)))| (r, Coord { x, y }))
}

fn sensor(i: &str) -> IResult<&str, Sensor> {
    tuple((
        tag("Sensor at "),
        coord,
        tag(": closest beacon is at "),
        coord,
    ))(i)
    .map(|(r, (_, location, _, closest))| {
        (
            r,
            Sensor {
                location,
                beacon: closest,
            },
        )
    })
}

fn sensors(i: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list0(newline, sensor)(i)
}

fn part1(input: &str) -> usize {
    const LINE: i32 = if cfg!(test) { 10 } else { 2_000_000 };

    let (_, sensors) = sensors(input).unwrap();

    let mut blackout = BTreeSet::new();
    for s in &sensors {
        let bo = s.blackout_region(LINE);
        if let Some((left, right)) = bo {
            for p in left..=right {
                blackout.insert(p);
            }
        }
    }

    blackout.len()
}

fn part2(input: &str) -> u64 {
    const RANGE: (i32, i32) = if cfg!(test) { (0, 20) } else { (0, 4_000_000) };

    let (_, mut sensors) = sensors(input).unwrap();
    sensors.sort_unstable();
    let sensors = sensors;

    let mut regions = Vec::new();
    for y in RANGE.0..=RANGE.1 {
        for s in &sensors {
            let bo = s.coverage_region(y);
            if let Some((mut left, mut right)) = bo {
                left = left.clamp(RANGE.0, RANGE.1);
                right = right.clamp(RANGE.0, RANGE.1);
                regions.push((left, right));
            }
        }
        regions.sort_unstable();

        let x = first_missing(&regions);
        if x <= RANGE.1 {
            return Coord { x, y }.tuning_frequency();
        }

        regions.clear();
    }

    panic!("not found")
}

// regions must be sorted first, otherwise this may return an incorrect value
fn first_missing(regions: &[(i32, i32)]) -> i32 {
    let mut first_missing = 0;
    for &(from, to) in regions {
        if from > first_missing {
            return first_missing;
        }
        if to >= first_missing {
            first_missing = to + 1;
        }
    }
    first_missing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(26, part1(TEST_INPUT));
        assert_eq!(56000011, part2(TEST_INPUT));
    }

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
}
