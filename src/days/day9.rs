use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

pub fn solve(input: String) -> String {
    let input = input.trim();
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct XY {
    x: i16,
    y: i16,
}

impl XY {
    fn abs(&self) -> XY {
        XY {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    fn unit(self) -> XY {
        let lambda = |a| match a {
            0 => 0,
            1.. => 1,
            _ => -1,
        };

        XY {
            x: lambda(self.x),
            y: lambda(self.y),
        }
    }
}

impl Add for XY {
    type Output = XY;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for XY {
    type Output = XY;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Rope {
    head: XY,
    tail: XY,
}

impl Rope {
    fn tail_moves(&self) -> bool {
        let delta = self.head - self.tail;
        let abs = delta.abs();
        abs.x > 1 || abs.y > 1
    }

    // take one step
    fn move_tail(&mut self) {
        let delta = self.head - self.tail;
        self.tail = self.tail + delta.unit();
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
}
use Direction::*;

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    len: i16,
}

impl std::str::FromStr for Move {
    type Err = ();

    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, len) = s.split_once(' ').unwrap();
        let len = len.parse().unwrap();
        Ok(match dir {
            "R" => Self { direction: Direction::Right, len },
            "L" => Self { direction: Direction::Right, len: -len },
            "U" => Self { direction: Direction::Up, len },
            "D" => Self { direction: Direction::Up, len: -len },
            _ => panic!(),
        })
    }
}

impl From<&Move> for XY {
    fn from(mov: &Move) -> Self {
        match mov.direction {
            Right => XY { x: mov.len, y: 0 },
            Up => XY { x: 0, y: mov.len },
        }
    }
}

fn part1(input: &str) -> usize {
    let moves: Vec<Move> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut rope = Rope {
        head: XY { x: 0, y: 0 },
        tail: XY { x: 0, y: 0 },
    };
    let mut visited = HashSet::from([rope.tail]);

    for mov in &moves {
        // head can actually move all at once
        rope.head = rope.head + mov.into();

        // tail may have multiple steps though
        while rope.tail_moves() {
            let delta = rope.head - rope.tail;
            rope.tail = rope.tail + delta.unit();
            visited.insert(rope.tail);
        }
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let moves: Vec<Move> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut ropes = [Rope::default(); 9];
    let mut visited = HashSet::from([XY::default()]);

    // in this rope physics, the head must move one step at a time, and the
    // tails only ever move once as a result
    for mov in &moves {
        let first_delta: XY = mov.into();
        for _ in 0..mov.len.abs() {
            let mut last_tail = ropes[0].head + first_delta.unit();

            // move the first 8 knots
            for rope in &mut ropes[..8] {
                rope.head = last_tail;
                if rope.tail_moves() {
                    rope.move_tail();
                }
                last_tail = rope.tail;
            }

            // move the last knot
            let last = ropes.last_mut().unwrap();
            last.head = last_tail;
            if last.tail_moves() {
                last.move_tail();
                visited.insert(last.tail);
            }
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            13,
            part1(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"
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
