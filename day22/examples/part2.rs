#![feature(slice_group_by)]

use std::{
    collections::HashMap,
    io::{self, Read},
    ops::Add,
};

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

enum Tile {
    Tree,
    Space,
}

enum Instruction {
    Move(i32),
    RotateLeft,
    RotateRight,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Face {
    Right,
    Down,
    Left,
    Up,
}

fn main() {
    let mut t = String::new();
    _ = io::stdin().lock().read_to_string(&mut t);

    let (raw_map, raw_instructions) = t.split_once("\n\n").unwrap();

    let map = raw_map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };

                match c {
                    ' ' => None,
                    '#' => Some((point, Tile::Tree)),
                    '.' => Some((point, Tile::Space)),
                    _ => panic!(),
                }
            })
        })
        .collect::<HashMap<_, _>>();

    let chars = raw_instructions.chars().collect::<Vec<_>>();
    let instructions = chars
        .group_by(|a, b| a.is_digit(10) && b.is_digit(10))
        .map(|c| match c.iter().collect::<String>().as_str() {
            "L" => Instruction::RotateLeft,
            "R" => Instruction::RotateRight,
            x => Instruction::Move(x.parse::<i32>().unwrap()),
        });

    let mut facing = [Face::Right, Face::Down, Face::Left, Face::Up];

    let (_, x) = meta_row(&map, 0);

    let mut point = Point { x, y: 0 };

    for instruction in instructions {
        match instruction {
            Instruction::RotateLeft => {
                facing.rotate_right(1);
            }
            Instruction::RotateRight => {
                facing.rotate_left(1);
            }
            Instruction::Move(v) => {
                let m = &facing[0];
                match m {
                    Face::Left | Face::Right => {
                        let (width, x_offset) = meta_row(&map, point.y);

                        let move_x = if m == &Face::Left { -1 } else { 1 };

                        for _ in 0..v {
                            let next_x =
                                (point.x + move_x - x_offset).rem_euclid(width as i32) + x_offset;
                            let next = Point {
                                x: next_x,
                                y: point.y,
                            };
                            match map.get(&next).unwrap() {
                                Tile::Tree => break,
                                Tile::Space => {
                                    point = next;
                                }
                            }
                        }
                    }
                    Face::Up | Face::Down => {
                        let (height, y_offset) = meta_col(&map, point.x);

                        let move_y = if m == &Face::Up { -1 } else { 1 };

                        for _ in 0..v {
                            let next_y =
                                (point.y + move_y - y_offset).rem_euclid(height as i32) + y_offset;
                            let next = Point {
                                x: point.x,
                                y: next_y,
                            };
                            match map.get(&next).unwrap() {
                                Tile::Tree => break,
                                Tile::Space => {
                                    point = next;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let result = 1000 * (point.y + 1) + 4 * (point.x + 1) + facing[0].clone() as i32;

    println!("{:?}", result);
}

fn meta_row(map: &HashMap<Point, Tile>, y: i32) -> (usize, i32) {
    let points = map.keys().filter(|p| p.y == y).collect::<Vec<_>>();
    (points.len(), points.iter().map(|p| p.x).min().unwrap())
}

fn meta_col(map: &HashMap<Point, Tile>, x: i32) -> (usize, i32) {
    let points = map.keys().filter(|p| p.x == x).collect::<Vec<_>>();
    (points.len(), points.iter().map(|p| p.y).min().unwrap())
}
