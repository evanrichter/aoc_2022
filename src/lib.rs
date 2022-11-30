mod day1;

#[rustfmt::skip]
const SOLVERS: &[&dyn Solve] = &[
    &day1::Solver,
];

pub trait Solve {
    fn solve(&self, input: String) -> String;
}

pub fn solve_all() -> anyhow::Result<()> {
    let aoc = emergence::AoC::new(2021)?;

    for (day, solver) in SOLVERS.iter().enumerate() {
        let day = day + 1;
        let input = aoc.read_or_fetch(day)?;
        let solution = solver.solve(input);
        println!("day {day} solution: {solution}");
    }

    Ok(())
}
