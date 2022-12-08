mod days {
    automod::dir!(pub "src/days");
}
pub use days::*;

pub fn solve_all() -> anyhow::Result<()> {
    let aoc = emergence::AoC::new(2022)?;

    for (day, solver) in SOLVERS.iter().enumerate() {
        let day = day + 1;
        let input = aoc.read_or_fetch(day)?;
        let solution = solver(input);
        println!("day {day} solution: {solution}");
    }

    Ok(())
}

const SOLVERS: &[fn(String) -> String] = &[
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
    day7::solve,
];
