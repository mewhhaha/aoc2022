#![feature(generic_arg_infer)]

use parse_display::{Display, FromStr};
use std::io::{self};
use std::ops::Add;

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[display("{x},{y},{z}")]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

type Space3D = Vec<Vec<Vec<i32>>>;

const LAVA: i32 = 0;
const AIR: i32 = 1;

fn main() {
    let points = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.parse::<Point>().unwrap())
        .collect::<Vec<_>>();

    let max_x = points.iter().map(|p| p.x).max().unwrap() as usize;
    let max_y = points.iter().map(|p| p.y).max().unwrap() as usize;
    let max_z = points.iter().map(|p| p.z).max().unwrap() as usize;

    let mut space: Space3D = vec![vec![vec![AIR; max_z + 1]; max_y + 1]; max_x + 1];

    let mut result = 0;

    for Point { x, y, z } in points.iter() {
        space[*x as usize][*y as usize][*z as usize] = LAVA;
    }

    for Point { x, y, z } in points {
        let p = Point {
            x: x as i32,
            y: y as i32,
            z: z as i32,
        };

        let s: i32 = ADJACENT_POSITONS
            .iter()
            .map(|a| *a + p)
            .map(|p| {
                let xs = space.get(p.x as usize)?;
                let ys = xs.get(p.y as usize)?;
                ys.get(p.z as usize)
            })
            .map(|v| v.unwrap_or(&1))
            .sum();

        result += s;
    }

    println!("{:?}", result);
}

const ADJACENT_POSITONS: [Point; 6] = [
    Point { x: 0, y: 0, z: 1 },
    Point { x: 0, y: 1, z: 0 },
    Point { x: 1, y: 0, z: 0 },
    Point { x: 0, y: 0, z: -1 },
    Point { x: 0, y: -1, z: 0 },
    Point { x: -1, y: 0, z: 0 },
];
