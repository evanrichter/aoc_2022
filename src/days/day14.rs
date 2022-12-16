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

#[derive(Debug)]
struct Rock {
    coords: Vec<Coord>,
}

impl Rock {
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

    let mut static_objects: BTreeSet<Coord> = BTreeSet::new();
    let mut sand_count = 0;

    // add each rock pixel to the static objects set
    for rock in rocks {
        for [from, to] in rock.coords.array_windows() {
            if from.x == to.x {
                for y in from.y.min(to.y)..=from.y.max(to.y) {
                    static_objects.insert(Coord { x: from.x, y });
                }
            } else {
                assert_eq!(from.y, to.y);
                for x in from.x.min(to.x)..=from.x.max(to.x) {
                    static_objects.insert(Coord { x, y: from.y });
                }
            }
        }
    }

    loop {
        // drop sand
        let mut sand = Coord { x: 500, y: 0 };

        let overlaps = |sand: &Coord| static_objects.contains(sand);

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
        static_objects.insert(sand.clone());
        sand_count += 1;

        // check if source is plugged
        if let Coord { x: 500, y: 0 } = sand {
            break;
        }
    }

    sand_count
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
