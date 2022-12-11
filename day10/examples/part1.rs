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
    let instructions = stdin
        .lines()
        .flatten()
        .map(|x| x.parse::<Instruction>().unwrap())
        .flat_map(|instruction| match instruction {
            Instruction::Noop => vec![Instruction::Noop],
            Instruction::AddX(v) => vec![Instruction::Noop, Instruction::AddX(v)],
        });

    let mut result = 0;
    let mut count_cycle = 20;
    let mut x = 1;

    for (current_cycle, instruction) in instructions.enumerate() {
        if current_cycle == count_cycle {
            result += x * count_cycle as i32;
            count_cycle += 40;
        }

        if let Instruction::AddX(v) = instruction {
            x += v;
        }
    }

    println!("{:?}", result);
}
