#![feature(slice_group_by)]
#![feature(bool_to_option)]

use std::{
    collections::HashMap,
    io::{self, Read},
    ops::{Add, Sub},
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Face {
    Right,
    Down,
    Left,
    Up,
}

const TILE_SIZE: i32 = 50;
const TILE_MAX_INDEX: i32 = 49;

/**
 * _12
 * _3_
 * 45_
 * 6__
 */
const TILE_ONE: Point = Point {
    x: 1 * TILE_SIZE,
    y: 0,
};
const TILE_TWO: Point = Point {
    x: 2 * TILE_SIZE,
    y: 0,
};
const TILE_THREE: Point = Point {
    x: 1 * TILE_SIZE,
    y: 1 * TILE_SIZE,
};
const TILE_FOUR: Point = Point {
    x: 0,
    y: 2 * TILE_SIZE,
};
const TILE_FIVE: Point = Point {
    x: 1 * TILE_SIZE,
    y: 2 * TILE_SIZE,
};
const TILE_SIX: Point = Point {
    x: 0,
    y: 3 * TILE_SIZE,
};

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

    let mut face = Face::Right;

    let mut position = TILE_ONE;

    for instruction in instructions {
        match instruction {
            Instruction::RotateLeft => {
                face = counter_clockwise(face);
            }
            Instruction::RotateRight => {
                face = clockwise(face);
            }
            Instruction::Move(v) => {
                for _ in 0..v {
                    let m = movement(&face);

                    let next = position + m;

                    match map.get(&next) {
                        Some(Tile::Tree) => break,
                        Some(Tile::Space) => {
                            position = next;
                        }
                        None => {
                            let t = tile_in(&position);
                            let relative = position - t;

                            let (edge, f) = match (t, face) {
                                // ONE LEFT - FOUR LEFT
                                (TILE_ONE, Face::Left) | (TILE_FOUR, Face::Left) => {
                                    let other_tile =
                                        if t == TILE_ONE { TILE_FOUR } else { TILE_ONE };
                                    let edge = Point {
                                        x: left_edge(other_tile),
                                        y: top_edge(other_tile) + upside_down(relative.y),
                                    };

                                    (edge, opposite(Face::Left))
                                }

                                // TWO RIGHT - FIVE RIGHT
                                (TILE_TWO, Face::Right) | (TILE_FIVE, Face::Right) => {
                                    let other_tile =
                                        if t == TILE_TWO { TILE_FIVE } else { TILE_TWO };
                                    let edge = Point {
                                        x: right_edge(other_tile),
                                        y: top_edge(other_tile) + upside_down(relative.y),
                                    };
                                    (edge, opposite(Face::Right))
                                }

                                // ONE UP - SIX LEFT
                                (TILE_ONE, Face::Up) => up_left(TILE_SIX, relative),
                                (TILE_SIX, Face::Left) => left_up(TILE_ONE, relative),

                                // FOUR UP - THREE LEFT
                                (TILE_FOUR, Face::Up) => up_left(TILE_THREE, relative),
                                (TILE_THREE, Face::Left) => left_up(TILE_FOUR, relative),

                                // TWO DOWN - THREE RIGHT
                                (TILE_TWO, Face::Down) => down_right(TILE_THREE, relative),
                                (TILE_THREE, Face::Right) => right_down(TILE_TWO, relative),

                                // FIVE DOWN - SIX RIGHT
                                (TILE_FIVE, Face::Down) => down_right(TILE_SIX, relative),
                                (TILE_SIX, Face::Right) => right_down(TILE_FIVE, relative),

                                // TWO UP - SIX DOWN
                                (TILE_TWO, Face::Up) => {
                                    let edge = Point {
                                        x: left_edge(TILE_SIX) + relative.x,
                                        y: bottom_edge(TILE_SIX),
                                    };

                                    (edge, Face::Up)
                                }
                                (TILE_SIX, Face::Down) => {
                                    let edge = Point {
                                        x: left_edge(TILE_TWO) + relative.x,
                                        y: top_edge(TILE_TWO),
                                    };
                                    (edge, Face::Down)
                                }

                                _ => panic!(),
                            };

                            match map.get(&edge) {
                                Some(Tile::Tree) => break,
                                Some(Tile::Space) => {
                                    position = edge;
                                    face = f;
                                }
                                _ => panic!(),
                            }
                        }
                    }
                }
            }
        }
    }

    let end = position + Point { x: 1, y: 1 };

    let result = 1000 * (end.y) + 4 * (end.x) + face as i32;

    println!("{:?}", result);
}

fn movement(f: &Face) -> Point {
    match f {
        Face::Up => Point { x: 0, y: -1 },
        Face::Down => Point { x: 0, y: 1 },
        Face::Right => Point { x: 1, y: 0 },
        Face::Left => Point { x: -1, y: 0 },
    }
}

fn tile_in(p: &Point) -> Point {
    [
        TILE_ONE, TILE_TWO, TILE_THREE, TILE_FOUR, TILE_FIVE, TILE_SIX,
    ]
    .iter()
    .find(|t| p.x >= t.x && p.x < t.x + TILE_SIZE && p.y >= t.y && p.y < t.y + TILE_SIZE)
    .unwrap()
    .clone()
}

fn right_edge(tile: Point) -> i32 {
    tile.x + TILE_MAX_INDEX
}

fn left_edge(tile: Point) -> i32 {
    tile.x
}

fn top_edge(tile: Point) -> i32 {
    tile.y
}

fn bottom_edge(tile: Point) -> i32 {
    tile.y + TILE_MAX_INDEX
}

fn upside_down(v: i32) -> i32 {
    TILE_MAX_INDEX - v
}

fn clockwise(face: Face) -> Face {
    match face {
        Face::Up => Face::Right,
        Face::Right => Face::Down,
        Face::Down => Face::Left,
        Face::Left => Face::Up,
    }
}

fn up_left(tile: Point, relative: Point) -> (Point, Face) {
    let edge = Point {
        x: left_edge(tile),
        y: top_edge(tile) + relative.x,
    };
    (edge, opposite(Face::Left))
}

fn left_up(tile: Point, relative: Point) -> (Point, Face) {
    let edge = Point {
        x: left_edge(tile) + relative.y,
        y: top_edge(tile),
    };
    (edge, opposite(Face::Up))
}

fn down_right(tile: Point, relative: Point) -> (Point, Face) {
    let edge = Point {
        x: right_edge(tile),
        y: top_edge(tile) + relative.x,
    };
    (edge, opposite(Face::Right))
}

fn right_down(tile: Point, relative: Point) -> (Point, Face) {
    let edge = Point {
        x: left_edge(tile) + relative.y,
        y: bottom_edge(tile),
    };
    (edge, opposite(Face::Down))
}

fn counter_clockwise(face: Face) -> Face {
    match face {
        Face::Up => Face::Left,
        Face::Right => Face::Up,
        Face::Down => Face::Right,
        Face::Left => Face::Down,
    }
}

fn opposite(face: Face) -> Face {
    match face {
        Face::Up => Face::Down,
        Face::Right => Face::Left,
        Face::Down => Face::Up,
        Face::Left => Face::Right,
    }
}
