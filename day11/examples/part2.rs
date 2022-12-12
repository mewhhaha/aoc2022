#![feature(array_zip)]

use parse_display::{Display, FromStr};
use std::io::{self, Read, Stdin};
use std::num::ParseIntError;

const N_MONKEYS: usize = 8;

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
struct Items(Vec<[u64; N_MONKEYS]>);

impl std::str::FromStr for Items {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s
            .split(", ")
            .map(|x| [x.parse::<u64>().unwrap(); N_MONKEYS])
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
    let mut monkeys: [Monkey; N_MONKEYS] = read_as_string(stdin)
        .split("\n\n")
        .map(|x| x.parse::<Monkey>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut inspections = [0 as u64; N_MONKEYS];

    let divisors = monkeys.clone().map(|m| m.test);

    let mod_divisors = |vs: &[u64; N_MONKEYS]| vs.zip(divisors).map(|(v, div)| v % div);

    let rounds = 0..10000;

    for _ in rounds {
        for n in 0..N_MONKEYS {
            inspections[n] += monkeys[n].items.0.len() as u64;

            let op = &monkeys[n].op;
            let if_true = monkeys[n].if_true;
            let if_false = monkeys[n].if_false;

            let throws = monkeys[n]
                .items
                .0
                .drain(..)
                .map(|item| {
                    let worry_level = mod_divisors(&item.map(|x| eval(x, op)));

                    let to = if worry_level[n] == 0 {
                        if_true
                    } else {
                        if_false
                    };

                    (to, worry_level)
                })
                .collect::<Vec<_>>();

            for (to, worry_level) in throws {
                monkeys[to].items.0.push(worry_level);
            }
        }
    }

    inspections.sort();
    inspections.reverse();
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
