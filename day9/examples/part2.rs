#![feature(bool_to_option)]

use parse_display::{Display, FromStr};
use std::{collections::HashSet, io, ops::Add};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Instruction {
    #[display("R {0}")]
    Right(i32),

    #[display("U {0}")]
    Up(i32),

    #[display("D {0}")]
    Down(i32),

    #[display("L {0}")]
    Left(i32),
}

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines().flatten();

    let mut head = Point { x: 0, y: 0 };
    let mut segments = vec![Point { x: 0, y: 0 }; 9];

    let mut visited = HashSet::new();
    visited.insert(head);

    for movement in lines.flat_map(parse_steps) {
        head = head + movement;

        segments = segments
            .iter()
            .scan(head, |head, segment| {
                let m = Point {
                    x: (head.x - segment.x).signum(),
                    y: (head.y - segment.y).signum(),
                };

                *head = if is_adjacent(*head, *segment) {
                    *segment
                } else {
                    *segment + m
                };

                Some(*head)
            })
            .collect::<Vec<_>>();

        if let Some(last) = segments.last() {
            visited.insert(*last);
        }
    }

    println!("{:?}", visited.len());
}

fn parse_steps<'a>(line: String) -> impl Iterator<Item = Point> {
    let input = match line.parse::<Instruction>().unwrap() {
        Instruction::Left(m) => Point { x: -m, y: 0 },
        Instruction::Right(m) => Point { x: m, y: 0 },
        Instruction::Up(m) => Point { x: 0, y: -m },
        Instruction::Down(m) => Point { x: 0, y: m },
    };

    let xs = (0..input.x.abs()).map(move |_| Point {
        x: input.x.signum(),
        y: 0,
    });

    let ys = (0..input.y.abs()).map(move |_| Point {
        x: 0,
        y: input.y.signum(),
    });

    xs.chain(ys)
}

fn is_adjacent(p1: Point, p2: Point) -> bool {
    let dx = (p1.x - p2.x).pow(2) as f32;
    let dy = (p1.y - p2.y).pow(2) as f32;
    (dx + dy).sqrt().floor() <= 1.0
}
