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
struct Items(Vec<Vec<u64>>);

impl std::str::FromStr for Items {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s
            .split(", ")
            .map(|x| vec![x.parse::<u64>().unwrap(); 8])
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

    let divisors = monkeys.iter().map(|m| m.test).collect::<Vec<_>>();

    let limit = |vs: &Vec<u64>| {
        vs.iter()
            .zip(divisors.iter())
            .map(|(v, div)| v % div)
            .collect::<Vec<_>>()
    };

    monkeys.iter_mut().for_each(|x| {
        x.items.0 = x.items.0.iter().map(limit).collect::<Vec<_>>();
    });

    let rounds = 0..10000;

    for _ in rounds {
        for i in 0..monkeys.len() {
            let Monkey {
                n,
                items,
                op,
                test: _,
                if_true,
                if_false,
            } = monkeys[i].clone();

            inspections[n] += items.0.len() as u64;

            for item in items.0.into_iter() {
                let worry_level = limit(&item.iter().map(|x| eval(*x, &op)).collect::<Vec<_>>());

                let to = if worry_level[i] == 0 {
                    if_true
                } else {
                    if_false
                };

                monkeys[to].items.0.push(worry_level);
            }

            monkeys[i].items.0.clear();
        }
    }

    inspections.sort();
    let result: u64 = inspections.into_iter().rev().take(2).product();
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
