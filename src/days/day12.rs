use std::collections::BTreeSet;

pub fn solve(input: &str) -> String {
    let input = input.trim();
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
struct GridEntry {
    index: usize,
    height: u8,
    // distance to goal
    distance: Distance,
}

#[derive(Debug, Default, PartialEq, Eq, Ord, Clone, Copy)]
enum Distance {
    #[default]
    Unknown,
    Known(usize),
}
use Distance::*;

impl Distance {
    fn unwrap(&self) -> usize {
        match self {
            Known(distance) => *distance,
            Unknown => panic!("unwrapped an unknown distance"),
        }
    }
}

impl From<(usize, u8)> for GridEntry {
    fn from((index, c): (usize, u8)) -> Self {
        Self {
            index,
            height: c,
            distance: Unknown,
        }
    }
}

impl PartialOrd for GridEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.distance, other.distance) {
            (Known(s), Known(o)) => s.partial_cmp(&o),
            _ => panic!("can't compare unknown distance"),
        }
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Known(s), Known(o)) => s.partial_cmp(&o),
            _ => panic!("can't compare unknown distance"),
        }
    }
}

fn part1(input: &str) -> usize {
    let width = input.find('\n').unwrap();
    let height = input.chars().filter(|c| c == &'\n').count();
    let mut grid: Vec<Vec<GridEntry>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| (width * y + x, c as u8).into())
                .collect()
        })
        .collect();

    let get_yx = |index: usize| {
        let y = index / width;
        let x = index % width;
        (y, x)
    };

    let up = |index| {
        let (y, x) = get_yx(index);
        Some((y.checked_sub(1)?, x))
    };

    let down = |index| {
        let (y, x) = get_yx(index);
        if y + 1 >= height {
            None
        } else {
            Some((y + 1, x))
        }
    };

    let left = |index| {
        let (y, x) = get_yx(index);
        Some((y, x.checked_sub(1)?))
    };

    let right = |index| {
        let (y, x) = get_yx(index);
        if x + 1 >= width {
            None
        } else {
            Some((y, x + 1))
        }
    };

    let start_index = grid
        .iter()
        .flatten()
        .position(|e| e.height == b'S')
        .unwrap();
    let end_index = grid
        .iter()
        .flatten()
        .position(|e| e.height == b'E')
        .unwrap();
    let start_xy = get_yx(start_index);
    let end_xy = get_yx(end_index);

    // set start height to 'a'
    grid[start_xy.0][start_xy.1].height = b'a';

    // set end height to 'z' and a known distance to goal (0)
    grid[end_xy.0][end_xy.1].height = b'z';
    grid[end_xy.0][end_xy.1].distance = Known(0);

    // priority queue starts with the end
    let mut queue = BTreeSet::from([grid[end_xy.0][end_xy.1]]);

    while let Some(current) = (&mut queue).pop_first() {
        let current_distance = current.distance.unwrap();
        if current.index == start_index {
            // found the start! return the smallest distance (must already be found)
            return current_distance;
        }

        fn climbable(us: GridEntry, other: &mut GridEntry) -> bool {
            // could climb up one to us or could fall down to us
            if us.height == other.height
                || us.height == other.height + 1
                || us.height < other.height
            {
                if other.distance == Unknown {
                    other.distance = Known(us.distance.unwrap() + 1);
                    return true;
                } else {
                    let od = other.distance.unwrap();
                    let possible = us.distance.unwrap() + 1;
                    if possible < od {
                        other.distance = Known(us.distance.unwrap() + 1);
                        return true;
                    }
                    return false;
                }
            } else {
                false
            }
        }

        // look up
        if let Some((y, x)) = up(current.index) {
            let up = &mut grid[y][x];
            if climbable(current, up) {
                queue.insert(*up);
            }
        }
        // look down
        if let Some((y, x)) = down(current.index) {
            let down = &mut grid[y][x];
            if climbable(current, down) {
                queue.insert(*down);
            }
        }
        // look left
        if let Some((y, x)) = left(current.index) {
            let left = &mut grid[y][x];
            if climbable(current, left) {
                queue.insert(*left);
            }
        }
        // look right
        if let Some((y, x)) = right(current.index) {
            let right = &mut grid[y][x];
            if climbable(current, right) {
                queue.insert(*right);
            }
        }
    }

    panic!("didn't find the start")
}

fn part2(input: &str) -> usize {
    let width = input.find('\n').unwrap();
    let height = input.chars().filter(|c| c == &'\n').count();
    let mut grid: Vec<Vec<GridEntry>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| (width * y + x, c as u8).into())
                .collect()
        })
        .collect();

    let get_yx = |index: usize| {
        let y = index / width;
        let x = index % width;
        (y, x)
    };

    let up = |index| {
        let (y, x) = get_yx(index);
        Some((y.checked_sub(1)?, x))
    };

    let down = |index| {
        let (y, x) = get_yx(index);
        if y + 1 >= height {
            None
        } else {
            Some((y + 1, x))
        }
    };

    let left = |index| {
        let (y, x) = get_yx(index);
        Some((y, x.checked_sub(1)?))
    };

    let right = |index| {
        let (y, x) = get_yx(index);
        if x + 1 >= width {
            None
        } else {
            Some((y, x + 1))
        }
    };

    let start_index = grid
        .iter()
        .flatten()
        .position(|e| e.height == b'S')
        .unwrap();
    let end_index = grid
        .iter()
        .flatten()
        .position(|e| e.height == b'E')
        .unwrap();
    let start_xy = get_yx(start_index);
    let end_xy = get_yx(end_index);

    // set start height to 'a'
    grid[start_xy.0][start_xy.1].height = b'a';

    // set end height to 'z' and a known distance to goal (0)
    grid[end_xy.0][end_xy.1].height = b'z';
    grid[end_xy.0][end_xy.1].distance = Known(0);

    // priority queue starts with the end
    let mut queue = BTreeSet::from([grid[end_xy.0][end_xy.1]]);

    let mut a_paths = Vec::new();

    while let Some(current) = (&mut queue).pop_first() {
        let current_distance = current.distance.unwrap();
        if current.height == b'a' {
            // found the start! return the smallest distance (must already be found)
            a_paths.push(current_distance);

            // we may be neighbor to other a paths! but we can continue because
            // this one is known to be better than them anyway
            continue;
        }

        fn climbable(us: GridEntry, other: &mut GridEntry) -> bool {
            // could climb up one to us or could fall down to us
            if us.height == other.height
                || us.height == other.height + 1
                || us.height < other.height
            {
                if other.distance == Unknown {
                    other.distance = Known(us.distance.unwrap() + 1);
                    return true;
                } else {
                    let od = other.distance.unwrap();
                    let possible = us.distance.unwrap() + 1;
                    if possible < od {
                        other.distance = Known(us.distance.unwrap() + 1);
                        return true;
                    }
                    return false;
                }
            } else {
                false
            }
        }

        // look up
        if let Some((y, x)) = up(current.index) {
            let up = &mut grid[y][x];
            if climbable(current, up) {
                queue.insert(*up);
            }
        }
        // look down
        if let Some((y, x)) = down(current.index) {
            let down = &mut grid[y][x];
            if climbable(current, down) {
                queue.insert(*down);
            }
        }
        // look left
        if let Some((y, x)) = left(current.index) {
            let left = &mut grid[y][x];
            if climbable(current, left) {
                queue.insert(*left);
            }
        }
        // look right
        if let Some((y, x)) = right(current.index) {
            let right = &mut grid[y][x];
            if climbable(current, right) {
                queue.insert(*right);
            }
        }
    }

    a_paths.sort_unstable();
    a_paths[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(31, part1(TEST_INPUT));
        assert_eq!(29, part2(TEST_INPUT));
    }

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
}
