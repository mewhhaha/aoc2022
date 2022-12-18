#![feature(generic_arg_infer)]

use std::collections::HashSet;
use std::io::{self, Read};
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug, PartialOrd, Ord)]
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

const HORIZONTAL_LINE: [Point; 5] = [
    Point { x: 0, y: 0 }, // Dummy point just for size alignment
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 2, y: 0 },
    Point { x: 3, y: 0 },
];
const PLUS_SIGN: [Point; 5] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: -1 },
    Point { x: 2, y: -1 },
    Point { x: 1, y: -2 },
];
const CORNER: [Point; 5] = [
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 2, y: 0 },
    Point { x: 2, y: -1 },
    Point { x: 2, y: -2 },
];
const VERTICAL_LINE: [Point; 5] = [
    Point { x: 0, y: 0 }, // Dummy point just for size alignment
    Point { x: 0, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: -2 },
    Point { x: 0, y: -3 },
];
const BOX: [Point; 5] = [
    Point { x: 0, y: 0 }, // Dummy point just for size alignment
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: -1 },
];

const SHAPES: [[Point; 5]; 5] = [HORIZONTAL_LINE, PLUS_SIGN, CORNER, VERTICAL_LINE, BOX];

fn main() {
    let mut t = String::new();
    let _ = io::stdin().lock().read_to_string(&mut t);

    let mut instructions = t.chars().cycle();
    let mut shapes = SHAPES.iter().cycle();

    let mut map = HashSet::<Point>::new();

    let mut top: i32 = 0;

    for _ in 0..2022 {
        let mut shape = shapes
            .next()
            .unwrap()
            .map(|p| p + Point { x: 3, y: top - 4 });

        loop {
            let push_offset = jet_push(instructions.next().unwrap());
            let pushed = shape.map(|p| p + push_offset);
            if !is_hit(&map, pushed.iter()) {
                shape = pushed;
            }

            let dropped = shape.map(|p| p + Point { x: 0, y: 1 });

            if !is_hit(&map, dropped.iter()) {
                shape = dropped;
            } else {
                map.extend(shape);
                let shape_top = shape.iter().map(|p| p.y).min().unwrap();
                top = top.min(shape_top);
                break;
            }
        }
    }

    println!("{:?}", top.abs())
}

fn is_hit<'a>(map: &HashSet<Point>, mut ps: impl Iterator<Item = &'a Point>) -> bool {
    ps.any(|p| is_outside(&p) || map.contains(&p))
}

fn is_outside(p: &Point) -> bool {
    p.x < 1 || p.x > 7 || p.y == 0
}

fn jet_push(i: char) -> Point {
    match i {
        '<' => Point { x: -1, y: 0 },
        '>' => Point { x: 1, y: 0 },
        _ => panic!(),
    }
}
