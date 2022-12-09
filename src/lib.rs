mod days {
    automod::dir!(pub "src/days");
}
pub use days::*;

pub fn solve_all() -> anyhow::Result<()> {
    let aoc = emergence::AoC::new(2022)?;

    for (day, solver) in SOLVERS.iter().enumerate() {
        let day = day + 1;
        let input = aoc.read_or_fetch(day)?;
        let solution = solver(&input);
        println!("day {day} solution: {solution}");
    }

    Ok(())
}

const SOLVERS: &[fn(&str) -> String] = &[
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
    day7::solve,
    day8::solve,
    day9::solve,
];

const STATIC_INPUT_SOLVERS: &[(fn(&str) -> String, &str)] = &[
    (day1::solve, include_str!("inputs/day01.txt")),
    (day2::solve, include_str!("inputs/day02.txt")),
    (day3::solve, include_str!("inputs/day03.txt")),
    (day4::solve, include_str!("inputs/day04.txt")),
    (day5::solve, include_str!("inputs/day05.txt")),
    (day6::solve, include_str!("inputs/day06.txt")),
    (day7::solve, include_str!("inputs/day07.txt")),
    (day8::solve, include_str!("inputs/day08.txt")),
    (day9::solve, include_str!("inputs/day09.txt")),
];

pub fn solve_with_static_input(day: usize) {
    let (solver, input) = STATIC_INPUT_SOLVERS[day - 1];
    let solution = solver(input);
    println!("day {day} solution: {solution}");
}
