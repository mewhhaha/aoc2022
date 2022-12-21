use parse_display::{Display, FromStr};
use std::{
    collections::HashMap,
    io::{self},
};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq)]
#[display("{name}: {yell}")]
struct Monkey {
    name: String,
    yell: Yell,
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq)]

enum Yell {
    #[display("{0} {1} {2}")]
    Equation(String, Op, String),
    #[display("{0}")]
    Number(i64),
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq)]
enum Op {
    #[display("+")]
    Add,
    #[display("-")]
    Sub,
    #[display("/")]
    Div,
    #[display("*")]
    Mul,
}

fn main() {
    let monkeys = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.parse::<Monkey>().unwrap())
        .map(|m| (m.name.clone(), m))
        .collect::<HashMap<_, _>>();

    let mut memo = HashMap::<String, i64>::new();

    let mut remaining = vec!["root".to_string()];

    while let Some(name) = remaining.pop() {
        if memo.contains_key(&name) {
            continue;
        }

        let m = monkeys.get(&name).unwrap();

        match &m.yell {
            Yell::Number(v) => {
                memo.insert(name, *v);
            }
            Yell::Equation(left_name, op, right_name) => {
                let left_value = memo.get(left_name);
                let right_value = memo.get(right_name);

                if let (Some(lhs), Some(rhs)) = (left_value, right_value) {
                    let a = *lhs; // Copying these so the borrow from memo can be dropped for the insert
                    let b = *rhs;
                    memo.insert(name, eval(a, op, b));
                } else {
                    remaining.push(name);
                    remaining.push(left_name.clone());
                    remaining.push(right_name.clone());
                }
            }
        }
    }

    let result = memo.get("root");
    println!("{:?}", result);
}

fn eval(lhs: i64, op: &Op, rhs: i64) -> i64 {
    match op {
        Op::Add => lhs + rhs,
        Op::Sub => lhs - rhs,
        Op::Div => lhs / rhs,
        Op::Mul => lhs * rhs,
    }
}
