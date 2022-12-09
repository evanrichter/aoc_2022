pub fn solve(input: &str) -> String {
    let input = input.trim();
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

fn parse_grid(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|row| row.as_bytes().iter().map(|b| *b as i8).collect())
        .collect()
}

// from https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let trees = parse_grid(input);
    let columns = trees.len();
    let mut treemap = vec![false; trees.len() * trees[0].len()];

    let mut seen = |x, y| {
        treemap[x + columns * y] = true;
    };

    for (row_index, row) in trees.iter().enumerate() {
        // look right
        let mut curr_height = -1;
        for (col_index, &tree) in row.iter().enumerate() {
            if tree > curr_height {
                curr_height = tree;
                seen(col_index, row_index);
            }
        }
        // look left
        let mut curr_height = -1;
        for (col_index, &tree) in row.iter().enumerate().rev() {
            if tree > curr_height {
                curr_height = tree;
                seen(col_index, row_index);
            }
        }
    }

    // look up & down
    let trees = transpose2(trees);

    for (col_index, row) in trees.iter().enumerate() {
        // look right
        let mut curr_height = -1;
        for (row_index, &tree) in row.iter().enumerate() {
            if tree > curr_height {
                curr_height = tree;
                seen(col_index, row_index);
            }
        }
        // look left
        let mut curr_height = -1;
        for (row_index, &tree) in row.iter().enumerate().rev() {
            if tree > curr_height {
                curr_height = tree;
                seen(col_index, row_index);
            }
        }
    }

    treemap.iter().filter(|f| **f).count()
}

fn part2(input: &str) -> usize {
    let trees = parse_grid(input);

    let mut best = 0;

    for (col_index, row) in trees.iter().enumerate() {
        if col_index == 0 || col_index == trees.len() - 1 {
            continue;
        }
        for (row_index, &tree) in row.iter().enumerate() {
            if row_index == 0 || row_index == row.len() - 1 {
                continue;
            }

            // look left
            let mut left = 1;
            for (num, ii) in (0..row_index).rev().enumerate() {
                left = num + 1;
                if row[ii] >= tree {
                    break;
                }
            }

            // look right
            let mut right = 1;
            for (num, ii) in ((row_index + 1)..row.len()).enumerate() {
                right = num + 1;
                if row[ii] >= tree {
                    break;
                }
            }

            // look up
            let mut up = 1;
            for (num, ii) in (0..col_index).rev().enumerate() {
                up = num + 1;
                if trees[ii][row_index] >= tree {
                    break;
                }
            }

            // look down
            let mut down = 1;
            for (num, ii) in ((col_index + 1)..trees.len()).enumerate() {
                down = num + 1;
                if trees[ii][row_index] >= tree {
                    break;
                }
            }

            let score = left * right * up * down;
            best = best.max(score);
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            21,
            part1(
                "30373
25512
65332
33549
35390"
            )
        );
        assert_eq!(
            8,
            part2(
                "30373
25512
65332
33549
35390"
            )
        );
    }
}
