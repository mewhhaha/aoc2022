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

    let print_cycle = 39;
    let mut row = "".to_string();
    let mut sprite_position = 1;

    for (current_cycle, instruction) in instructions.enumerate() {
        let row_position = current_cycle as i32 % (print_cycle + 1);
        let c = match row_position - sprite_position {
            diff if diff >= -1 && diff <= 1 => '#',
            _ => '.',
        };
        row.push(c);

        if row_position == print_cycle {
            println!("{:?}", row);
            row = "".to_string();
        }

        if let Instruction::AddX(v) = instruction {
            sprite_position += v;
        }
    }
}
