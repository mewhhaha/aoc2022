#![feature(slice_group_by)]

use parse_display::{Display, FromStr};
use std::collections::HashSet;
use std::io;
use std::ops::Add;

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[display("{x},{y}")]
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

const DESTINATIONS: [Point; 3] = [
    Point { x: 0, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: 1, y: 1 },
];

fn main() {
    let mut graph = io::stdin()
        .lines()
        .flatten()
        .flat_map(parse_rock)
        .collect::<HashSet<_>>();

    let floor_y = graph
        .iter()
        .max_by(|a, b| a.y.cmp(&b.y))
        .map(|a| a.y)
        .unwrap()
        + 2;

    let mut count = 0;
    let source = Point { x: 500, y: 0 };
    let mut sand = source.clone();
    while !graph.contains(&source) {
        let destination = DESTINATIONS
            .iter()
            .map(|p| *p + sand)
            .filter(|p| !graph.contains(&p) && p.y != floor_y)
            .next();

        if let Some(p) = destination {
            sand = p;
        } else {
            graph.insert(sand);
            sand = source.clone();
            count += 1;
        }
    }

    println!("{:?}", count)
}

fn parse_rock(s: String) -> Vec<Point> {
    let mut result = Vec::new();
    let mut points = s.split(" -> ").map(|x| x.parse::<Point>().unwrap());
    let mut current = points.next().unwrap();

    for point in points {
        let range_x = if current.x < point.x {
            current.x..=point.x
        } else {
            point.x..=current.x
        };
        let range_y = if current.y < point.y {
            current.y..=point.y
        } else {
            point.y..=current.y
        };

        result.extend(range_x.flat_map(|x| range_y.clone().map(move |y| Point { x, y })));
        current = point;
    }

    return result;
}
