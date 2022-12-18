use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32 as u32_parser},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, tuple},
    IResult,
};
use std::collections::BTreeSet;
use std::collections::HashMap;

const MINUTES: usize = 30;

pub fn solve(input: &str) -> String {
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

#[derive(Debug)]
struct ParsedValve {
    name: String,
    flow_rate: u32,
    connections: Vec<String>,
}

fn connections(i: &str) -> IResult<&str, Vec<String>> {
    alt((
        pair(
            tag("; tunnels lead to valves "),
            separated_list1(tag(", "), map(alpha1, |a: &str| a.to_string())),
        ),
        pair(
            tag("; tunnel leads to valve "),
            map(alpha1, |a: &str| vec![a.to_string()]),
        ),
    ))(i)
    .map(|(r, (_, s))| (r, s))
}

fn valve(i: &str) -> IResult<&str, ParsedValve> {
    tuple((
        tag("Valve "),
        map(alpha1, |a: &str| a.to_string()),
        tag(" has flow rate="),
        u32_parser,
        connections,
    ))(i)
    .map(|(r, (_, name, _, flow_rate, connections))| {
        (
            r,
            ParsedValve {
                name,
                flow_rate,
                connections,
            },
        )
    })
}

fn valves(i: &str) -> IResult<&str, Vec<ParsedValve>> {
    separated_list1(newline, valve)(i)
}

#[derive(Debug, Clone)]
struct Valve {
    // Some(val) when available,
    // None when already opened
    flow_rate: Option<u32>,
    connections: Vec<usize>,
}

fn part1(input: &str) -> u32 {
    let (_, parsed_valves) = valves(input).unwrap();

    let mut valves = Vec::new();
    let mut valve_ids: HashMap<&str, usize> = HashMap::new();

    for v in &parsed_valves {
        assert!(v.connections.len() < 256);
        valve_ids.insert(v.name.as_str(), valves.len());
        valves.push(Valve {
            flow_rate: Some(v.flow_rate),
            connections: Vec::new(),
        });
    }

    for (valve, parsed) in valves.iter_mut().zip(parsed_valves.iter()) {
        for name in &parsed.connections {
            valve.connections.push(valve_ids[name.as_str()])
        }
    }

    for v in &valves {
        println!("{v:?}");
    }

    let mut threads = Vec::new();
    for _ in 0usize..std::thread::available_parallelism().unwrap().into() {
        let valves = valves.clone();
        let jh = std::thread::spawn(move || fuzz(valves, usize::MAX));
        threads.push(jh);
    }
    for t in threads {
        t.join().unwrap();
    }
    0
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Open,
    Move(u8),
}
use Action::*;

fn fuzz(valves: Vec<Valve>, iters: usize) -> u32 {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let mut best = 0;
    let mut seen = BTreeSet::new();
    let mut corpus = Vec::from([([Open; MINUTES], 100)]);

    for iter in 0..iters {
        //println!("FUZZ RUN {iter}");
        let mut fuzz_valves = valves.clone();

        let (mut actions, score) = corpus.choose_weighted(&mut rng, |item| item.1).unwrap();
        //let (mut actions, score) = corpus.choose(&mut rng).unwrap();

        // mutate
        const CORRUPTION_FACTOR: u8 = 8;
        for _ in 0..=(rng.gen::<u8>() % CORRUPTION_FACTOR) {
            // pick an index
            let action = actions.choose_mut(&mut rng).unwrap();
            *action = match action {
                Open => Move(rng.gen::<u8>()),
                Move(m) => {
                    if rng.gen() {
                        Open
                    } else {
                        if rng.gen() {
                            Move(m.wrapping_add(1))
                        } else {
                            Move(m.wrapping_sub(1))
                        }
                    }
                }
            };
        }

        let result = fuzz_one(&mut fuzz_valves, &actions);

        //if result > best {
        if !seen.contains(&result) {
            corpus.push((actions.clone(), result));
            best = best.max(result);
            seen.insert(result);
            println!("best: {best} corpus size: {}", corpus.len());
        }
    }
    best
}

fn fuzz_one(valves: &mut [Valve], actions: &[Action]) -> u32 {
    let mut steam = 0;
    let mut valve = 0;

    for (minute, action) in actions.iter().take(MINUTES).enumerate() {
        let minute = minute + 1;

        match action {
            Action::Open => {
                if let Some(flow) = valves[valve].flow_rate.take() {
                    steam += (MINUTES - minute) * flow as usize;
                }
            }
            Action::Move(idx) => {
                let conns = &valves[valve].connections;
                valve = conns[*idx as usize % conns.len()];
            }
        }
    }

    steam as u32
}

fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(1651, part1(TEST_INPUT));
        assert_eq!(0, part2(TEST_INPUT));
    }

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
}
