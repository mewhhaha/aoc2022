use parse_display::{Display, FromStr};
use std::io;

#[derive(Display, FromStr, PartialEq, Debug)]
enum Instruction {
    #[display("noop")]
    Noop,

    #[display("addx {0}")]
    AddX(i32),
}

fn main() {
    let stdin = io::stdin();
    let instructions =
        stdin
            .lines()
            .flatten()
            .flat_map(|x| match x.parse::<Instruction>().unwrap() {
                Instruction::Noop => vec![Instruction::Noop],
                Instruction::AddX(v) => vec![Instruction::Noop, Instruction::AddX(v)],
            });

    let mut result = 0;
    let mut count_cycle = 20;
    let mut current_cycle = 0;
    let mut x = 1;

    for instruction in instructions {
        current_cycle += 1;
        if current_cycle == count_cycle {
            result += x * count_cycle;
            count_cycle += 40;
        }

        if let Instruction::AddX(v) = instruction {
            x += v;
        }
    }

    println!("{:?}", result);
}
