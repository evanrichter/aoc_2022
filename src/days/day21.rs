use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

pub fn solve(input: &str) -> String {
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

#[derive(Debug, Clone, Copy)]
enum AstNode<'m> {
    Immediate(i64),
    Add(&'m str, &'m str),
    Sub(&'m str, &'m str),
    Mul(&'m str, &'m str),
    Div(&'m str, &'m str),
}

#[derive(Debug, Clone, Copy)]
struct Monkey<'m> {
    name: &'m str,
    node: AstNode<'m>,
}

fn monkeys(s: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(
        nom::character::complete::newline,
        map(separated_pair(alpha1, tag(": "), node), |(name, node)| {
            Monkey { name, node }
        }),
    )(s)
}

fn node(s: &str) -> IResult<&str, AstNode> {
    alt((
        map(separated_pair(alpha1, tag(" + "), alpha1), |(a, b)| {
            AstNode::Add(a, b)
        }),
        map(separated_pair(alpha1, tag(" - "), alpha1), |(a, b)| {
            AstNode::Sub(a, b)
        }),
        map(separated_pair(alpha1, tag(" * "), alpha1), |(a, b)| {
            AstNode::Mul(a, b)
        }),
        map(separated_pair(alpha1, tag(" / "), alpha1), |(a, b)| {
            AstNode::Div(a, b)
        }),
        map(digit1, |d: &str| AstNode::Immediate(d.parse().unwrap())),
    ))(s)
}

fn find_monkey<'a>(monkeys: &'a [Monkey], name: &str) -> Monkey<'a> {
    *monkeys.iter().find(|m| m.name == name).unwrap()
}

fn eval(monkeys: &[Monkey], name: &str) -> i64 {
    let node = find_monkey(monkeys, name).node;
    match node {
        AstNode::Immediate(i) => i,
        AstNode::Add(a, b) => eval(monkeys, a) + eval(monkeys, b),
        AstNode::Sub(a, b) => eval(monkeys, a) - eval(monkeys, b),
        AstNode::Mul(a, b) => eval(monkeys, a) * eval(monkeys, b),
        AstNode::Div(a, b) => eval(monkeys, a) / eval(monkeys, b),
    }
}

#[derive(Debug, Clone)]
enum SymbolicExpr {
    Symbolic,
    Concrete(i64),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
}

impl SymbolicExpr {
    fn reduce(self) -> Self {
        use SymbolicExpr::*;

        match self {
            Symbolic => self,
            Concrete(_) => self,
            Add(a, b) => {
                let a = a.reduce();
                let b = b.reduce();
                if let (&Concrete(a), &Concrete(b)) = (&a, &b) {
                    Concrete(a + b)
                } else {
                    Add(Box::new(a), Box::new(b))
                }
            }
            Sub(a, b) => {
                let a = a.reduce();
                let b = b.reduce();
                if let (&Concrete(a), &Concrete(b)) = (&a, &b) {
                    Concrete(a - b)
                } else {
                    Sub(Box::new(a), Box::new(b))
                }
            }
            Mul(a, b) => {
                let a = a.reduce();
                let b = b.reduce();
                if let (&Concrete(a), &Concrete(b)) = (&a, &b) {
                    Concrete(a * b)
                } else {
                    Mul(Box::new(a), Box::new(b))
                }
            }
            Div(a, b) => {
                let a = a.reduce();
                let b = b.reduce();
                if let (&Concrete(a), &Concrete(b)) = (&a, &b) {
                    Concrete(a / b)
                } else {
                    Div(Box::new(a), Box::new(b))
                }
            }
        }
    }
}

fn build_expr(monkeys: &[Monkey], name: &str) -> SymbolicExpr {
    use SymbolicExpr::*;
    if name == "humn" {
        return Symbolic;
    }

    let node = find_monkey(monkeys, name).node;
    match node {
        AstNode::Immediate(i) => Concrete(i),
        AstNode::Add(a, b) => Add(
            Box::new(build_expr(monkeys, a)),
            Box::new(build_expr(monkeys, b)),
        ),
        AstNode::Sub(a, b) => Sub(
            Box::new(build_expr(monkeys, a)),
            Box::new(build_expr(monkeys, b)),
        ),
        AstNode::Mul(a, b) => Mul(
            Box::new(build_expr(monkeys, a)),
            Box::new(build_expr(monkeys, b)),
        ),
        AstNode::Div(a, b) => Div(
            Box::new(build_expr(monkeys, a)),
            Box::new(build_expr(monkeys, b)),
        ),
    }
}

fn balance_expr(expr: SymbolicExpr, target: i64) -> i64 {
    use SymbolicExpr::*;
    let (expr, target) = match expr {
        Symbolic => return target,
        Add(a, b) => match (*a, *b) {
            (expr, Concrete(concrete)) => (expr, target - concrete),
            (Concrete(concrete), expr) => (expr, target - concrete),
            _ => panic!("one side of expression needs to be concrete"),
        },
        Mul(a, b) => match (*a, *b) {
            (expr, Concrete(concrete)) => (expr, target / concrete),
            (Concrete(concrete), expr) => (expr, target / concrete),
            _ => panic!("one side of expression needs to be concrete"),
        },
        Sub(a, b) => match (*a, *b) {
            (expr, Concrete(concrete)) => (expr, target + concrete),
            (Concrete(concrete), expr) => (expr, concrete - target),
            _ => panic!("one side of expression needs to be concrete"),
        },
        Div(a, b) => match (*a, *b) {
            (expr, Concrete(concrete)) => (expr, target * concrete),
            (Concrete(concrete), expr) => (expr, concrete / target),
            _ => panic!("one side of expression needs to be concrete"),
        },
        Concrete(_) => panic!(),
    };

    balance_expr(expr, target)
}

fn solve_expr(left: SymbolicExpr, right: SymbolicExpr) -> i64 {
    use SymbolicExpr::*;
    // reduce the left and right sides
    let left = left.reduce();
    let right = right.reduce();

    let (expr, concrete) = match (left, right) {
        (expr, Concrete(concrete)) => (expr, concrete),
        (Concrete(concrete), expr) => (expr, concrete),
        _ => panic!("one side of expression needs to be concrete"),
    };

    balance_expr(expr, concrete)
}

fn part1(input: &str) -> i64 {
    let monkeys = monkeys(input).unwrap().1;
    eval(&monkeys, "root")
}

fn part2(input: &str) -> i64 {
    let monkeys = monkeys(input).unwrap().1;

    let root = find_monkey(&monkeys, "root");
    let AstNode::Add(a, b) = root.node else { panic!() };
    solve_expr(build_expr(&monkeys, a), build_expr(&monkeys, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(152, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(301, part2(TEST_INPUT));
    }

    const TEST_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
}
