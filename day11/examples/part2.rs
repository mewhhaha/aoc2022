#![feature(array_zip)]

use parse_display::{Display, FromStr};
use std::io::{self, Read, Stdin};
use std::num::ParseIntError;

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display(
    "Monkey {n}:
  Starting items: {items}
  Operation: new = old {op}
  Test: divisible by {test}
    If true: throw to monkey {if_true}
    If false: throw to monkey {if_false}"
)]
struct Monkey {
    n: usize,
    items: Items,
    op: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(PartialEq, Debug, Clone)]
struct Items(Vec<u64>);

impl std::str::FromStr for Items {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Ok(Items(words))
    }
}

impl std::fmt::Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "") // This is wrong but I just need this for the trait
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{symbol} {rhs}")]
struct Operation {
    rhs: Value,
    symbol: Symbol,
}

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
enum Value {
    #[display("old")]
    Old,

    #[display("{0}")]
    Number(u64),
}

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
enum Symbol {
    #[display("+")]
    Add,
    #[display("*")]
    Multiply,
}

fn main() {
    let stdin = io::stdin();
    let mut monkeys = read_as_string(stdin)
        .split("\n\n")
        .map(|x| x.parse::<Monkey>().unwrap())
        .collect::<Vec<_>>();

    let mut inspections = vec![0 as u64; monkeys.len()];

    let lcm = monkeys.iter().map(|m| m.test).product::<u64>();

    for _ in 0..10000 {
        for n in 0..monkeys.len() {
            let Monkey {
                n: _,
                items: Items(items),
                test,
                op,
                if_true,
                if_false,
            } = &mut monkeys[n];

            inspections[n] += items.len() as u64;

            let throws = items
                .drain(..)
                .map(|item| {
                    let worry_level = eval(item, &op) % lcm;

                    let to = if worry_level % *test == 0 {
                        if_true.clone()
                    } else {
                        if_false.clone()
                    };

                    (to, worry_level)
                })
                .collect::<Vec<_>>();

            for (to, worry_level) in throws {
                monkeys[to].items.0.push(worry_level);
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    let result: u64 = inspections[0] * inspections[1];
    println!("{:?}", result);
}

fn eval(i: u64, op: &Operation) -> u64 {
    let rhs = value(i, &op.rhs);
    match &op.symbol {
        Symbol::Add => i + rhs,
        Symbol::Multiply => i * rhs,
    }
}

fn value(i: u64, value: &Value) -> u64 {
    match value {
        Value::Number(v) => *v,
        Value::Old => i,
    }
}

fn read_as_string(stdin: Stdin) -> String {
    stdin
        .lock()
        .bytes()
        .filter_map(|u| u.map(|x| x as char).ok())
        .collect::<String>()
}
