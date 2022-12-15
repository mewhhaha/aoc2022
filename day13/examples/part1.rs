#![feature(slice_group_by)]

use std::collections::VecDeque;
use std::io::{self, Read};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq)]

enum Symbol {
    #[display("[")]
    OpenList,

    #[display("]")]
    CloseList,
    #[display("{0}")]
    Number(u32),
}

fn main() {
    let mut text = String::new();
    _ = io::stdin().lock().read_to_string(&mut text);

    let pairs = text.split("\n\n").filter_map(|t| t.split_once("\n"));

    let mut result = 0;

    for (i, (left, right)) in pairs.enumerate() {
        let left_packet = parse_packet(left);
        let right_packet = parse_packet(right);

        if is_less_than(left_packet, right_packet) {
            result += i + 1;
        }
    }

    println!("{:?}", result);
}

fn is_less_than(mut left: VecDeque<Symbol>, mut right: VecDeque<Symbol>) -> bool {
    while let (Some(next_left), Some(next_right)) = (left.pop_front(), right.pop_front()) {
        match (next_left, next_right) {
            (x, y) if x == y => continue,
            (Symbol::OpenList, Symbol::Number(r)) => {
                right.push_front(Symbol::CloseList);
                right.push_front(Symbol::Number(r));
                continue;
            }
            (Symbol::Number(r), Symbol::OpenList) => {
                left.push_front(Symbol::CloseList);
                left.push_front(Symbol::Number(r));
                continue;
            }

            (Symbol::CloseList, _) => break,
            (Symbol::Number(l), Symbol::Number(r)) => return l < r,

            _ => return false,
        };
    }

    return true;
}

fn parse_packet(s: &str) -> VecDeque<Symbol> {
    s.chars()
        .collect::<Vec<_>>()
        .group_by(|a, b| a.is_digit(10) && b.is_digit(10))
        .map(|x| x.iter().collect::<String>())
        .filter(|x| x != ",")
        .map(|x| x.parse::<Symbol>().unwrap())
        .collect::<VecDeque<_>>()
}
