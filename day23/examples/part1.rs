use std::{
    collections::{HashMap, HashSet},
    io::{self},
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
        .collect::<HashSet<_>>();

    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        let mut proposals = HashMap::<Point, Option<Point>>::new();

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
            map.remove(&from);
            map.insert(to);
        }

        directions.rotate_left(1);
    }

    let (min_x, max_x, min_y, max_y) = map.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), p| {
            (
                min_x.min(p.x),
                max_x.max(p.x),
                min_y.min(p.y),
                max_y.max(p.y),
            )
        },
    );

    let result = (max_x + 1 - min_x) * (max_y + 1 - min_y) - map.len() as i32;

    println!("{:?}", result);
}

fn is_alone(map: &HashSet<Point>, center: &Point) -> bool {
    ADJACENT
        .iter()
        .map(|p| *p + *center)
        .all(|p| match map.get(&p) {
            Some(_) => false,
            _ => true,
        })
}

fn consider_moving(map: &HashSet<Point>, center: &Point, direction: &Direction) -> Option<Point> {
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
