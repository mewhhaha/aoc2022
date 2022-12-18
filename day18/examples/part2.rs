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
const FREE: i32 = 2;

fn main() {
    let points = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.parse::<Point>().unwrap())
        .collect::<Vec<_>>();

    let max_x = points.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x as usize;
    let max_y = points.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y as usize;
    let max_z = points.iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap().z as usize;

    let mut space: Space3D = vec![vec![vec![AIR; max_z + 1]; max_y + 1]; max_x + 1];

    for Point { x, y, z } in points.iter() {
        space[*x as usize][*y as usize][*z as usize] = LAVA;
    }

    // Update any AIR with FREE if they're adjacent to a FREE or the edge
    // until there's nothing left to update
    loop {
        let mut updated = false;

        for (x, y, z) in coordinates(max_x, max_y, max_z) {
            if space[x][y][z] != AIR {
                continue;
            }

            let p = Point {
                x: x as i32,
                y: y as i32,
                z: z as i32,
            };

            let free: bool = ADJACENT_POSITONS
                .iter()
                .map(|a| *a + p)
                .map(|p| {
                    let xs = space.get(p.x as usize)?;
                    let ys = xs.get(p.y as usize)?;
                    ys.get(p.z as usize)
                })
                .any(|v| v == None || v == Some(&FREE));

            if free {
                space[x][y][z] = FREE;
                updated = true;
            }
        }

        if !updated {
            break;
        }
    }

    // Make each empty space an empty space and treat each unreachable area as a block
    for (x, y, z) in coordinates(max_x, max_y, max_z) {
        match space[x][y][z] {
            FREE => space[x][y][z] = AIR,
            AIR => space[x][y][z] = LAVA,
            _ => (),
        }
    }

    let mut result = 0;

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

fn coordinates(
    max_x: usize,
    max_y: usize,
    max_z: usize,
) -> impl Iterator<Item = (usize, usize, usize)> {
    (0..=max_x)
        .flat_map(move |x| (0..=max_y).flat_map(move |y| (0..=max_z).map(move |z| (x, y, z))))
}
