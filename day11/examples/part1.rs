use parse_display::{Display, FromStr};
use std::io::{self, Read, Stdin};
use std::num::ParseIntError;

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display(
    "Monkey {n}:
  Starting items: {items}
  Operation: new = {op}
  Test: divisible by {test}
    If true: throw to monkey {if_true}
    If false: throw to monkey {if_false}"
)]
struct Monkey {
    n: usize,
    items: Items,
    op: Operation,
    test: u32,
    if_true: usize,
    if_false: usize,
}

#[derive(PartialEq, Debug, Clone)]
struct Items(Vec<u32>);

impl std::str::FromStr for Items {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s
            .split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        Ok(Items(words))
    }
}

impl std::fmt::Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .0
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{}", items)
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{lhs} {symbol} {rhs}")]
struct Operation {
    lhs: Value,
    rhs: Value,
    symbol: Symbol,
}

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
enum Value {
    #[display("old")]
    Old,

    #[display("{0}")]
    Number(u32),
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

    let mut inspections = vec![0 as u32; monkeys.len()];

    let rounds = 0..20;

    for _ in rounds {
        for i in 0..monkeys.len() {
            let Monkey {
                n,
                items,
                op,
                test,
                if_true,
                if_false,
            } = monkeys[i].clone();

            inspections[n] += items.0.len() as u32;

            for item in items.0.into_iter() {
                let worry_level = eval(item, &op) / 3;

                let to = if worry_level % test == 0 {
                    if_true
                } else {
                    if_false
                };

                monkeys[to].items.0.push(worry_level);
            }

            monkeys[i].items.0.clear();
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    let result: u32 = inspections[0] * inspections[1];
    println!("{:?}", result);
}

fn eval(i: u32, op: &Operation) -> u32 {
    let lhs = value(i, &op.lhs);
    let rhs = value(i, &op.rhs);
    match &op.symbol {
        Symbol::Add => lhs + rhs,
        Symbol::Multiply => lhs * rhs,
    }
}

fn value(i: u32, value: &Value) -> u32 {
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
