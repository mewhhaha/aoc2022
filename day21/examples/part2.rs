use parse_display::{Display, FromStr};
use std::{
    collections::{HashMap, HashSet},
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

// Seems from testing that only the lhs is related to humn
fn main() {
    let monkeys = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.parse::<Monkey>().unwrap())
        .map(|m| (m.name.clone(), m))
        .collect::<HashMap<_, _>>();

    let (mut memo, human_related) = run_monkeys(
        &HashMap::<String, i64>::new(),
        vec!["root".to_string()],
        &monkeys,
    );

    memo.retain(|name, _| !human_related.contains(name));

    let monkey = monkeys.get("root").unwrap();

    let (lhs, _, rhs) = try_equation(&monkey.yell).unwrap();

    let start_value = memo.get(&lhs).or_else(|| memo.get(&rhs)).unwrap();

    let humn_value = solve_humn(&memo, *start_value * 2, &"root".to_string(), &monkeys);

    println!("{:}", humn_value);
}

fn solve_humn(
    evaled: &HashMap<String, i64>,
    humn: i64,
    name: &String,
    monkeys: &HashMap<String, Monkey>,
) -> i64 {
    if name == "humn" {
        return humn;
    }

    let monkey = monkeys.get(name).unwrap();

    match &monkey.yell {
        Yell::Number(_) => panic!(),
        Yell::Equation(left_name, op, right_name) => {
            let (next_name, next_value) = match (evaled.get(left_name), evaled.get(right_name)) {
                (Some(lhs), _) => (right_name, solve_lhs(*lhs, op, humn)),
                (_, Some(rhs)) => (left_name, solve_rhs(humn, op, *rhs)),
                _ => panic!(),
            };

            return solve_humn(evaled, next_value, next_name, monkeys);
        }
    }
}

fn solve_lhs(value: i64, op: &Op, eq: i64) -> i64 {
    match op {
        Op::Add => eq - value,
        Op::Sub => -(eq - value),
        Op::Div => value / eq,
        Op::Mul => eq / value,
    }
}

fn solve_rhs(eq: i64, op: &Op, value: i64) -> i64 {
    match op {
        Op::Add => eq - value,
        Op::Sub => eq + value,
        Op::Div => value * eq,
        Op::Mul => eq / value,
    }
}

fn run_monkeys(
    start_memo: &HashMap<String, i64>,
    mut remaining: Vec<String>,
    monkeys: &HashMap<String, Monkey>,
) -> (HashMap<String, i64>, HashSet<String>) {
    let mut memo = start_memo.clone();

    let mut related = HashSet::<String>::from_iter(vec!["humn".to_string()]);

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

                    if related.contains(left_name) || related.contains(right_name) {
                        related.insert(name.clone());
                    }

                    memo.insert(name, eval(a, op, b));
                } else {
                    remaining.push(name);
                    remaining.push(left_name.clone());
                    remaining.push(right_name.clone());
                }
            }
        }
    }

    (memo, related)
}

fn try_equation(yell: &Yell) -> Option<(String, Op, String)> {
    match yell {
        Yell::Number(_) => None,
        Yell::Equation(left_name, op, right_name) => {
            Some((left_name.clone(), op.clone(), right_name.clone()))
        }
    }
}

fn eval(lhs: i64, op: &Op, rhs: i64) -> i64 {
    match op {
        Op::Add => lhs + rhs,
        Op::Sub => lhs - rhs,
        Op::Div => lhs / rhs,
        Op::Mul => lhs * rhs,
    }
}
