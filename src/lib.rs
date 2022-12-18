#![feature(array_windows)]

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
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
    day10::solve,
    day11::solve,
    day12::solve,
    day13::solve,
    day14::solve,
    day15::solve,
];

const STATIC_INPUT_SOLVERS: &[(fn(&str) -> String, &str)] = &[
    (day01::solve, include_str!("inputs/day01.txt")),
    (day02::solve, include_str!("inputs/day02.txt")),
    (day03::solve, include_str!("inputs/day03.txt")),
    (day04::solve, include_str!("inputs/day04.txt")),
    (day05::solve, include_str!("inputs/day05.txt")),
    (day06::solve, include_str!("inputs/day06.txt")),
    (day07::solve, include_str!("inputs/day07.txt")),
    (day08::solve, include_str!("inputs/day08.txt")),
    (day09::solve, include_str!("inputs/day09.txt")),
    (day10::solve, include_str!("inputs/day10.txt")),
    (day11::solve, include_str!("inputs/day11.txt")),
    (day12::solve, include_str!("inputs/day12.txt")),
    (day13::solve, include_str!("inputs/day13.txt")),
    (day14::solve, include_str!("inputs/day14.txt")),
    (day15::solve, include_str!("inputs/day15.txt")),
];

pub fn solve_with_static_input(day: usize) {
    let (solver, input) = STATIC_INPUT_SOLVERS[day - 1];
    let solution = solver(input);
    println!("day {day} solution: {solution}");
}
