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

    let print_cycle = 40;
    let mut current_cycle = 0;
    let mut row = "".to_string();
    let mut sprite_position = 1;

    for instruction in instructions {
        let diff = current_cycle - sprite_position;
        let c = if diff >= -1 && diff <= 1 { '#' } else { '.' };
        row.push(c);

        current_cycle += 1;

        if current_cycle == print_cycle {
            println!("{:?}", row);
            current_cycle = 0;
            row = "".to_string();
        }

        if let Instruction::AddX(v) = instruction {
            sprite_position += v;
        }
    }
}
