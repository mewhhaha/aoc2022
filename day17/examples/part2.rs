#![feature(generic_arg_infer)]

use std::collections::HashSet;
use std::io::{self, Read};
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
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
const CRAZY_NUMBER: i64 = 1000000000000;

fn main() {
    let mut t = String::new();
    let _ = io::stdin().lock().read_to_string(&mut t);

    let mut instructions = t.chars().cycle();
    let mut shapes = SHAPES.iter().cycle();

    let mut map = HashSet::<Point>::new();

    let mut top_values = vec![];
    let mut top: i64 = 0;
    let mut prev_top: i64 = top;
    let mut diff_top: i64 = -1;
    let mut prev_rocks: i64 = 0;
    let mut diff_rocks: i64 = -1;

    let mut number_of_instructions = 0;

    'outer: for rocks in 0..10000 {
        top_values.push(top);
        let mut shape = shapes
            .next()
            .unwrap()
            .map(|p| p + Point { x: 3, y: top - 4 });

        loop {
            let push_offset = jet_push(instructions.next().unwrap());
            number_of_instructions += 1;
            if number_of_instructions % t.len() == 0 {
                // Whenever the instructions are reset, there seems to be a consistent pattern
                // when it comes to how much the shapes and top value will increase until the next round of instructions
                let temp_diff_top = top - prev_top;
                let temp_diff_rocks = rocks - prev_rocks;
                if temp_diff_top == diff_top && temp_diff_rocks == diff_rocks {
                    let mod_value = top_values[(CRAZY_NUMBER % diff_rocks) as usize];
                    let multiple_value = (CRAZY_NUMBER / diff_rocks) * diff_top;
                    top = mod_value + multiple_value;
                    break 'outer;
                }
                diff_top = temp_diff_top;
                diff_rocks = temp_diff_rocks;
                prev_top = top;
                prev_rocks = rocks;
            }
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
