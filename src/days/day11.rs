use std::collections::VecDeque;
use std::str::FromStr;

pub fn solve(input: &str) -> String {
    let input = input.trim();
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

fn part1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    let modulus: u64 = monkeys.iter().map(|m| m.test.divisible_by).product();
    for _ in 0..20 {
        round::<true>(&mut monkeys, modulus);
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.total_inspected).collect();
    inspections.sort_unstable();

    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn part2(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    let modulus: u64 = monkeys.iter().map(|m| m.test.divisible_by).product();

    for _ in 0..10_000 {
        round::<false>(&mut monkeys, modulus);
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.total_inspected).collect();
    inspections.sort_unstable();

    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn round<const DIV_BY_3: bool>(monkeys: &mut [Monkey], modulus: u64) {
    for monkey_id in 0..monkeys.len() {
        // get the monkey out
        let mut monkey = core::mem::take(&mut monkeys[monkey_id]);
        monkey.total_inspected += monkey.items.len();

        // do stuff with the items
        while let Some(mut item) = monkey.items.pop_front() {
            // guard operation by the combined modulus of all monkeys
            item %= modulus;
            // operation
            item = monkey.operation.apply(&item);
            if DIV_BY_3 {
                item /= 3;
            }

            // test
            let target_monkey = if item % monkey.test.divisible_by == 0 {
                monkey.test.true_monkey
            } else {
                monkey.test.false_monkey
            };

            monkeys[target_monkey].items.push_back(item);
        }

        // put the monkey back
        monkeys[monkey_id] = monkey;
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_str| monkey_str.parse().unwrap())
        .collect()
}

#[derive(Debug, Default)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    total_inspected: usize,
}

#[derive(Debug)]
enum Operand {
    Old,
    Num(u64),
}
use Operand::*;

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Self::Old)
        } else {
            Ok(Self::Num(s.parse().unwrap()))
        }
    }
}

#[derive(Debug)]
enum Operation {
    Multiply(Operand),
    Add(Operand),
}
use Operation::*;

impl Operation {
    fn apply(&self, old: &u64) -> u64 {
        match self {
            Multiply(Old) => old * old,
            Multiply(Num(n)) => old * n,
            Add(Old) => old + old,
            Add(Num(n)) => old + n,
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Self::Add(Old)
    }
}

#[derive(Debug, Default)]
struct Test {
    divisible_by: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        assert!(lines.next().unwrap().starts_with("Monkey "));

        // items
        let (_, items) = lines.next().unwrap().split_once("items: ").unwrap();
        let items = items.split(", ").map(|i| i.parse().unwrap()).collect();

        // operation
        let (_, operation) = lines
            .next()
            .unwrap()
            .split_once("Operation: new = old ")
            .unwrap();
        let operation = if let Some((_, num)) = operation.split_once("* ") {
            Operation::Multiply(num.parse().unwrap())
        } else {
            let (_, num) = operation.split_once("+ ").unwrap();
            Operation::Add(num.parse().unwrap())
        };

        // test
        let divisible_by = lines
            .next()
            .unwrap()
            .split_once("divisible by ")
            .map(|(_, db)| db.parse().unwrap())
            .unwrap();
        let true_monkey = lines
            .next()
            .unwrap()
            .split_once("true: throw to monkey ")
            .map(|(_, monkey)| monkey.parse().unwrap())
            .unwrap();
        let false_monkey = lines
            .next()
            .unwrap()
            .split_once("false: throw to monkey ")
            .map(|(_, monkey)| monkey.parse().unwrap())
            .unwrap();
        let test = Test {
            divisible_by,
            true_monkey,
            false_monkey,
        };

        Ok(Self {
            items,
            operation,
            test,
            total_inspected: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(10605, part1(TEST_INPUT));
        assert_eq!(2713310158, part2(TEST_INPUT));
    }

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
}
