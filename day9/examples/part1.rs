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
    let mut tail = Point { x: 0, y: 0 };

    let mut visited = HashSet::new();
    visited.insert(head);

    for movement in lines.flat_map(parse_steps) {
        head = head + movement;

        let m = Point {
            x: (head.x - tail.x).signum(),
            y: (head.y - tail.y).signum(),
        };

        if !is_adjacent(head, tail) {
            tail = tail + m;
        }

        visited.insert(tail);
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
