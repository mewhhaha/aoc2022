use parse_display::{Display, FromStr};
use std::io;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("move {amount} from {a} to {b}")]
struct Instruction {
    amount: usize,
    a: usize,
    b: usize,
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines().flatten().collect::<Vec<_>>();

    let (header, rest) = lines.split_at(8);
    let (_, instructions) = rest.split_at(2);

    let mut rows = header
        .iter()
        .map(parse_row)
        .map(|row| row.into_iter())
        .collect::<Vec<_>>();

    let mut grid = [0; 9].map(|_| {
        rows.iter_mut()
            .filter_map(|row| row.next().unwrap())
            .collect::<String>()
    });

    for Instruction { amount, a, b } in instructions.iter().map(parse_instruction) {
        let from = a - 1;
        let to = b - 1;
        let (left, from_result) = grid[from].split_at(amount);
        let mut to_result = reverse_string(left);
        to_result.push_str(grid[to].as_str());

        grid[from] = from_result.to_string();
        grid[to] = to_result;
    }

    let result = grid
        .iter()
        .map(|col| col.chars().next())
        .flatten()
        .collect::<String>();
    print!("{:?}", result);
}

fn reverse_string(s: &str) -> String {
    return s.chars().rev().collect::<String>();
}

fn parse_instruction(s: &String) -> Instruction {
    s.parse().expect("it to succeed")
}

fn parse_row(s: &String) -> [Option<char>; 9] {
    let mut chars = s.chars();
    [1, 2, 3, 4, 5, 6, 7, 8, 9].map(|i| {
        chars
            .nth(if i == 1 { 1 } else { 3 })
            .and_then(|c| if c != ' ' { Some(c) } else { None })
    })
}
