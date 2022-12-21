use std::collections::BTreeMap;

use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many1, IResult};

pub fn solve(input: &str) -> String {
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    F,
    R,
    A,
}
use Block::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Piece {
    // the pieces to draw
    rows: &'static [[Block; 9]],
}

const FLAT: Piece = Piece {
    rows: &[[R, A, A, F, F, F, F, A, R]],
};
const PLUS: Piece = Piece {
    rows: &[
        [R, A, A, A, F, A, A, A, R],
        [R, A, A, F, F, F, A, A, R],
        [R, A, A, A, F, A, A, A, R],
    ],
};
const L: Piece = Piece {
    rows: &[
        [R, A, A, A, A, F, A, A, R],
        [R, A, A, A, A, F, A, A, R],
        [R, A, A, F, F, F, A, A, R],
    ],
};
const LONG: Piece = Piece {
    rows: &[
        [R, A, A, F, A, A, A, A, R],
        [R, A, A, F, A, A, A, A, R],
        [R, A, A, F, A, A, A, A, R],
        [R, A, A, F, A, A, A, A, R],
    ],
};
const BOX: Piece = Piece {
    #[rustfmt::skip]
    rows: &[
        [R, A, A, F, F, A, A, A, R],
        [R, A, A, F, F, A, A, A, R],
    ],
};
const PIECES: [Piece; 5] = [FLAT, PLUS, L, LONG, BOX];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Jet {
    Left,
    Right,
}

fn jets(i: &str) -> IResult<&str, Vec<Jet>> {
    many1(alt((
        map(tag("<"), |_| Jet::Left),
        map(tag(">"), |_| Jet::Right),
    )))(i)
}

#[allow(dead_code)]
fn print_grid(grid: &[[Block; 9]]) {
    for layer in grid.iter().rev() {
        let s: String = layer
            .iter()
            .map(|b| match b {
                F => '@',
                R => '#',
                A => '.',
            })
            .collect();
        println!("{}", s);
    }
    println!();
}

fn line_hash(line: &[Block; 9]) -> u8 {
    let mut hash = 0;
    for (idx, b) in line.iter().take(8).enumerate() {
        if *b == A {
            hash |= 1 << idx;
        }
    }
    hash
}

fn grid_hash(grid: &[[Block; 9]], next_piece: usize, next_jet: usize) -> Vec<u8> {
    grid.iter()
        .map(line_hash)
        .chain(next_piece.to_le_bytes().iter().cloned())
        .chain(next_jet.to_le_bytes().iter().cloned())
        .collect()
}

fn tetris(input: &str, pieces: u64) -> u64 {
    let (_, jets) = jets(input).unwrap();

    // loop the jets forever
    let mut jets = jets.iter().enumerate().cycle().peekable();
    // grid state (Y grows up)
    const FLOOR: [Block; 9] = [R; 9];
    const AIR: [Block; 9] = [R, A, A, A, A, A, A, A, R];
    let mut grid = Vec::new();
    grid.push(FLOOR);

    let mut lines_past: u64 = 0;
    let mut seen: BTreeMap<Vec<u8>, (u64, u64)> = BTreeMap::new();

    let mut pieces_cycle = PIECES.iter().enumerate().cycle().peekable();

    let mut taken = 0;
    let mut check_cycle = false;
    while taken < pieces {
        let (piece_index, piece) = pieces_cycle.next().unwrap();

        if check_cycle {
            let hash = grid_hash(&grid, piece_index, jets.peek().unwrap().0);
            match seen.get(&hash) {
                Some((lines, pieces_taken)) => {
                    let cycles_possible = (pieces - taken) / pieces_taken;
                    lines_past += cycles_possible * lines;
                    taken += cycles_possible * pieces_taken;
                }
                None => panic!("no cycle"),
            }
            check_cycle = false;
        }

        // find the floor
        loop {
            let layer = grid.pop().unwrap();
            if layer != AIR {
                grid.push(layer);
                break;
            }
        }

        // spawn in the piece on top of the floor
        grid.push(AIR);
        grid.push(AIR);
        grid.push(AIR);
        for layer in piece.rows.iter().rev() {
            grid.push(*layer);
        }

        loop {
            // try push with jet
            let (_, jet) = jets.next().unwrap();
            use Jet::*;

            // store moved rows temporarily, only apply the move if all succeed
            let mut moved = Vec::new();
            let mut depth = 0;

            'move_check: for layer in grid.iter().rev() {
                if !layer.contains(&F) {
                    depth += 1;
                    continue;
                }

                for blocks in layer.windows(2) {
                    // match if a falling rock runs into a rock
                    if matches!((blocks, *jet), (&[R, F], Left) | (&[F, R], Right)) {
                        moved = Vec::new(); // clear out the moves
                        break 'move_check;
                    }
                }

                // compute the moved layer
                let mut new_layer = *layer;
                let (new_skip, old_skip) = match jet {
                    Left => (0, 1),
                    Right => (1, 0),
                };

                for (new, old) in new_layer
                    .iter_mut()
                    .skip(new_skip)
                    .zip(layer.iter().skip(old_skip))
                {
                    if matches!((*new, *old), (A, F) | (F, F)) {
                        *new = F;
                    } else if matches!((*new, *old), (F, A) | (F, R)) {
                        *new = A;
                    }
                }
                moved.insert(0, new_layer);

                if moved.len() == piece.rows.len() {
                    // checked enough rows
                    break;
                }
            }

            let grid_len = grid.len();
            grid[grid_len - depth - moved.len()..][..moved.len()].copy_from_slice(&moved);

            // try move down
            // store both outcomes of: moving down, and turning to stone,
            // applying the correct move after figuring it out
            let mut fallen = Vec::new();
            let mut stoned = Vec::new();
            let mut skipped = 0;

            // track falling blocks from previous iters
            let mut fall_mask = [false; 9];
            let mut stop_falling = false;
            for layer in grid.iter().rev().skip_while(|l| {
                skipped += 1;
                !l.contains(&F)
            }) {
                // check falling blocks from 1 above
                stop_falling = stop_falling
                    || layer
                        .iter()
                        .zip(fall_mask.iter())
                        .any(|(block, falling)| *block == R && *falling);

                // turn falling to rock
                stoned.insert(0, layer.map(|b| if b == F { R } else { b }));

                let mut fallen_layer = *layer;
                for (i, f) in fall_mask.iter().enumerate() {
                    if fallen_layer[i] == F {
                        fallen_layer[i] = A;
                    }
                    if *f {
                        fallen_layer[i] = F;
                    }
                }
                fallen.insert(0, fallen_layer);

                fall_mask = layer.map(|b| b == F);

                if !fall_mask.iter().any(|f| *f) {
                    // break if there's nothing falling in this layer
                    break;
                }
            }

            if stop_falling {
                let grid_len = grid.len();
                grid[grid_len - skipped + 1 - stoned.len()..][..stoned.len()]
                    .copy_from_slice(&stoned);
                break;
            } else {
                let grid_len = grid.len();
                grid[grid_len - skipped - fallen.len() + 1..][..fallen.len()]
                    .copy_from_slice(&fallen);
            }
        }

        if let Some(mut new_floor) = grid.iter().skip(1).position(|l| *l == FLOOR) {
            // found another floor! increase the count to this index
            lines_past += new_floor as u64 + 1;
            // we can destroy the previous layers to save allocation size
            new_floor += 2;
            grid.retain(|_| {
                new_floor = new_floor.saturating_sub(1);
                new_floor == 0
            });

            let next_piece = pieces_cycle.peek().unwrap().0;
            let next_jet = jets.peek().unwrap().0;
            let hash = grid_hash(&grid, next_piece, next_jet);
            seen.entry(hash)
                .and_modify(|entry| {
                    check_cycle = true;
                    entry.0 = lines_past - entry.0;
                    entry.1 = taken - entry.1;
                })
                .or_insert((lines_past, taken));
        }

        taken += 1;
    }

    // pop off any air
    loop {
        let layer = grid.pop().unwrap();
        if layer != AIR {
            grid.push(layer);
            break;
        }
    }

    // subtract 1 for the (first) floor
    lines_past + grid.len() as u64 - 1
}

fn part1(input: &str) -> u64 {
    tetris(input, 2022)
}

fn part2(input: &str) -> u64 {
    tetris(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3068, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1514285714288, part2(TEST_INPUT));
    }

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";
}
