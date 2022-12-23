use std::{
    io,
    ops::{Add, Sub},
};

use rustc_hash::{FxHashMap, FxHashSet};

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

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug, PartialOrd, Ord)]
enum Direction {
    North,
    West,
    East,
    South,
}

const ADJACENT: [Point; 8] = [
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 1 },
    Point { x: -1, y: -1 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: 1 },
];

fn main() {
    let mut map = io::stdin()
        .lines()
        .flatten()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '.' => None,
                    '#' => Some(Point {
                        x: x as i32,
                        y: y as i32,
                    }),
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<FxHashSet<_>>();

    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for round in 1.. {
        let mut equilibrium = true;
        let mut proposals = FxHashMap::<Point, Option<Point>>::default();

        for position in map.iter() {
            if is_alone(&map, &position) {
                continue;
            }

            for direction in directions {
                if let Some(next) = consider_moving(&map, &position, &direction) {
                    match proposals.get(&next) {
                        Some(Some(_)) => proposals.insert(next, None),
                        _ => proposals.insert(next, Some(*position)),
                    };
                    break;
                }
            }
        }

        for (to, from) in proposals.into_iter().filter_map(|(k, v)| v.map(|w| (k, w))) {
            equilibrium = false;
            map.remove(&from);
            map.insert(to);
        }

        directions.rotate_left(1);

        if equilibrium {
            println!("{:?}", round);
            break;
        }
    }
}

fn is_alone(map: &FxHashSet<Point>, center: &Point) -> bool {
    ADJACENT
        .iter()
        .map(|p| *p + *center)
        .all(|p| match map.get(&p) {
            Some(_) => false,
            _ => true,
        })
}

fn consider_moving(map: &FxHashSet<Point>, center: &Point, direction: &Direction) -> Option<Point> {
    let offset = match direction {
        Direction::North => Point { x: 0, y: -1 },
        Direction::South => Point { x: 0, y: 1 },
        Direction::West => Point { x: -1, y: 0 },
        Direction::East => Point { x: 1, y: 0 },
    };

    let is_free = ADJACENT
        .iter()
        .filter(|p| manhattan(p, &offset) <= 1)
        .map(|p| *p + *center)
        .all(|p| match map.get(&p) {
            Some(_) => false,
            _ => true,
        });

    if is_free {
        Some(*center + offset)
    } else {
        None
    }
}

fn manhattan(a: &Point, b: &Point) -> i32 {
    let delta = *a - *b;
    delta.x.abs() + delta.y.abs()
}
