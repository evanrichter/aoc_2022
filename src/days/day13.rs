use nom::{
    bytes::complete::tag,
    character::complete::{newline, u8 as u8_parser},
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, pair, terminated},
    IResult,
};
use std::cmp::{Ordering, PartialEq, PartialOrd};

pub fn solve(input: &str) -> String {
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

#[derive(Debug)]
struct Pair {
    l: List,
    r: List,
}

type List = Vec<Expr>;

#[derive(Debug, Clone, Eq, Ord)]
enum Expr {
    Num(u8),
    List(List),
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        use Expr as Ex;
        match (self, other) {
            (Ex::Num(a), Ex::Num(b)) => a.eq(b),
            (Ex::List(a), Ex::List(b)) => a.eq(b),
            (Ex::Num(a), Ex::List(b)) | (Ex::List(b), Ex::Num(a)) => {
                let a = vec![Ex::Num(*a)];
                a.eq(b)
            }
        }
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        use Expr as Ex;
        match (self, rhs) {
            (Ex::Num(a), Ex::Num(b)) => a.partial_cmp(&b),
            (Ex::List(a), Ex::List(b)) => a.partial_cmp(b),
            (Ex::Num(a), Ex::List(b)) => {
                let a = vec![Ex::Num(*a)];
                a.partial_cmp(b)
            }
            (Ex::List(a), Ex::Num(b)) => {
                let b = vec![Ex::Num(*b)];
                a.partial_cmp(&b)
            }
        }
    }
}

fn expression_parser(i: &str) -> IResult<&str, Expr> {
    if i.starts_with("[") {
        let (rest, list) = list_parser(i)?;
        Ok((rest, Expr::List(list)))
    } else {
        let (rest, num) = u8_parser(i)?;
        Ok((rest, Expr::Num(num)))
    }
}

fn list_parser(i: &str) -> IResult<&str, List> {
    let (rest, items) = delimited(
        tag("["),
        separated_list0(tag(","), expression_parser),
        tag("]"),
    )(i)?;
    Ok((rest, items))
}

fn pair_parser(i: &str) -> IResult<&str, Pair> {
    let (rest, (l, r)) = pair(
        terminated(list_parser, newline),
        terminated(list_parser, opt(newline)),
    )(i)?;
    Ok((
        rest,
        Pair {
            l: l.into(),
            r: r.into(),
        },
    ))
}

fn all_pairs(i: &str) -> IResult<&str, Vec<Pair>> {
    separated_list0(newline, pair_parser)(i)
}

fn all_packets(i: &str) -> IResult<&str, Vec<List>> {
    separated_list0(pair(newline, opt(newline)), list_parser)(i)
}

fn part1(input: &str) -> usize {
    let (_, pairs) = all_pairs(input).unwrap();
    pairs
        .iter()
        .enumerate()
        .filter_map(|(index, p)| (p.l <= p.r).then_some(index + 1))
        .sum()
}

fn part2(input: &str) -> usize {
    let (_, mut packets) = all_packets(input).unwrap();
    let marker1 = vec![Expr::List(vec![Expr::Num(2)])];
    let marker2 = vec![Expr::List(vec![Expr::Num(6)])];
    packets.push(marker1.clone());
    packets.push(marker2.clone());

    packets.sort_unstable();

    let m1 = packets.iter().position(|p| p == &marker1).unwrap() + 1;
    let m2 = packets.iter().position(|p| p == &marker2).unwrap() + 1;

    m1 * m2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(13, part1(TEST_INPUT));
        assert_eq!(140, part2(TEST_INPUT));
    }

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
}
