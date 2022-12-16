#![feature(slice_group_by)]

use std::io;
use std::{cmp::Ordering, collections::VecDeque};

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
    let mut packets = io::stdin()
        .lines()
        .flatten()
        .filter(|t| t != "")
        .map(parse_packet)
        .collect::<Vec<_>>();

    let two = parse_packet("[[2]]".to_string());
    let six = parse_packet("[[6]]".to_string());

    packets.push(two.clone());
    packets.push(six.clone());

    packets.sort_by(compare);

    let two = packets.iter().position(|x| x == &two);
    let six = packets.iter().position(|x| x == &six);

    if let (Some(a), Some(b)) = (two, six) {
        println!("{:?}", (a + 1) * (b + 1));
    }
}

fn compare(l: &VecDeque<Symbol>, r: &VecDeque<Symbol>) -> Ordering {
    let mut left = l.clone();
    let mut right = r.clone();

    while let (Some(next_left), Some(next_right)) = (left.pop_front(), right.pop_front()) {
        match (next_left, next_right) {
            (x, y) if x == y => (),
            (Symbol::OpenList, Symbol::Number(n)) => {
                right.push_front(Symbol::CloseList);
                right.push_front(Symbol::Number(n));
            }
            (Symbol::Number(n), Symbol::OpenList) => {
                left.push_front(Symbol::CloseList);
                left.push_front(Symbol::Number(n));
            }

            (Symbol::Number(a), Symbol::Number(b)) => return a.cmp(&b),
            (Symbol::CloseList, _) => return Ordering::Less,
            _ => return Ordering::Greater,
        };
    }

    return Ordering::Less;
}

fn parse_packet(s: String) -> VecDeque<Symbol> {
    s.chars()
        .collect::<Vec<_>>()
        .group_by(|a, b| a.is_digit(10) && b.is_digit(10))
        .map(|x| x.iter().collect::<String>())
        .filter(|x| x != ",")
        .map(|x| x.parse::<Symbol>().unwrap())
        .collect::<VecDeque<_>>()
}
