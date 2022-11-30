#[derive(Clone, Copy)]
pub struct Solver;

impl super::Solve for Solver {
    fn solve(&self, input: String) -> String {
        // part 1
        let depths: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();
        let part1: u32 = depths.windows(2).filter(|w| w[1] > w[0]).count();

        // part 2
        let sums_of_three: Vec<u32> = depths.windows(3).map(|w| w.iter().sum()).collect();
        let part2: u32 = sums_of_three.windows(2).filter(|w| w[1] > w[0]).count();
        format!("{part1}, {part2}")
    }
}
