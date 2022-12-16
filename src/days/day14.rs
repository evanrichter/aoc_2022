use nom::{
    bytes::complete::tag,
    character::complete::{i32 as i32_parser, newline},
    multi::separated_list0,
    multi::separated_list1,
    sequence::separated_pair,
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
    fn between(&self, &[ref a, ref b]: &[Coord; 2]) -> bool {
        let same_x = self.x == a.x || self.x == b.x;
        let same_y = self.y == a.y || self.y == b.y;
        let (s, smaller, bigger) = match (same_x, same_y) {
            (true, true) => return true,    // on an end-point
            (false, false) => return false, // not on same line
            // some x matches
            (true, false) => {
                let smaller_y = b.y.min(a.y);
                let bigger_y = b.y.max(a.y);
                if smaller_y == bigger_y {
                    return false;
                }
                (self.y, smaller_y, bigger_y)
            }
            // some y matches
            (false, true) => {
                let smaller_x = b.x.min(a.x);
                let bigger_x = b.x.max(a.x);
                if smaller_x == bigger_x {
                    return false;
                }
                (self.x, smaller_x, bigger_x)
            }
        };

        s >= smaller && s <= bigger
    }
}

#[derive(Debug)]
struct Rock {
    coords: Vec<Coord>,
}

impl Rock {
    fn overlaps(&self, coord: &Coord) -> bool {
        for pair in self.coords.array_windows() {
            if coord.between(pair) {
                return true;
            }
        }
        false
    }

    // y coord of lowest rock
    fn lowest_point(&self) -> i32 {
        self.coords.iter().map(|c| c.y).max().unwrap()
    }
}

fn coord(i: &str) -> IResult<&str, Coord> {
    separated_pair(i32_parser, tag(","), i32_parser)(i).map(|(r, (x, y))| (r, Coord { x, y }))
}

fn rock(i: &str) -> IResult<&str, Rock> {
    separated_list1(tag(" -> "), coord)(i).map(|(r, coords)| (r, Rock { coords }))
}

fn rocks(i: &str) -> IResult<&str, Vec<Rock>> {
    separated_list0(newline, rock)(i)
}

fn part1(input: &str) -> usize {
    let (_, rocks) = rocks(input).unwrap();
    simulate(&rocks)
}

fn part2(input: &str) -> usize {
    let (_, mut rocks) = rocks(input).unwrap();
    let lowest_rock = rocks.iter().map(|r| r.lowest_point()).max().unwrap();
    let floor_y = lowest_rock + 2;

    // no need to simulate infinite floor since a pyramid is the farthest the
    // sand will go
    let floor_left = 500 - floor_y - 10;
    let floor_right = 500 + floor_y + 10;
    let floor = Rock {
        coords: vec![
            Coord {
                x: floor_left,
                y: floor_y,
            },
            Coord {
                x: floor_right,
                y: floor_y,
            },
        ],
    };

    rocks.push(floor);

    simulate(&rocks)
}

fn simulate(rocks: &[Rock]) -> usize {
    let lowest_rock = rocks.iter().map(|r| r.lowest_point()).max().unwrap();

    let mut old_sand: BTreeSet<Coord> = BTreeSet::new();

    loop {
        // drop sand
        let mut sand = Coord { x: 500, y: 0 };

        let overlaps =
            |sand: &Coord| old_sand.contains(sand) || rocks.iter().any(|r| r.overlaps(sand));

        while sand.y < lowest_rock {
            let mut test = sand.clone();

            // test down one
            test.y += 1;
            if !overlaps(&test) {
                // found a good spot to fall
                sand = test;
                continue;
            }

            // try to the left
            test.x -= 1;
            if !overlaps(&test) {
                sand = test;
                continue;
            }

            // try to the right
            test.x += 2;
            if !overlaps(&test) {
                sand = test;
                continue;
            }

            // no test spots worked
            break;
        }

        if sand.y == lowest_rock {
            // would fall forever
            break;
        }

        // save stopped sand
        old_sand.insert(sand.clone());

        // check if source is plugged
        if let Coord { x: 500, y: 0 } = sand {
            break;
        }
    }

    old_sand.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(24, part1(TEST_INPUT));
        assert_eq!(93, part2(TEST_INPUT));
    }

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
}
