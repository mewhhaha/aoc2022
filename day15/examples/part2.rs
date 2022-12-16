#![feature(slice_group_by)]

use parse_display::{Display, FromStr};
use std::io;
use std::ops::{Add, Sub};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[display("x={x}, y={y}")]
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[display("Sensor at {sensor}: closest beacon is at {beacon}")]
struct Sensor {
    sensor: Point,
    beacon: Point,
}

fn main() {
    let sensors = io::stdin()
        .lines()
        .filter_map(|line| line.unwrap().parse::<Sensor>().ok())
        .collect::<Vec<_>>();

    let limit = 4000000;

    let mut potential_positions = Vec::<Point>::new();

    for Sensor { sensor, beacon } in sensors.iter() {
        let d = distance(&sensor, &beacon) + 1; // The empty one has to be one distance from the edge

        let a = (sensor.x - d..sensor.x)
            .zip(sensor.y..sensor.y + d)
            .map(|(x, y)| Point { x, y })
            .collect::<Vec<_>>(); // I used a HashSet here before but it was super slow
        let b = rotate(&a);
        let c = rotate(&b);
        let d = rotate(&c);

        potential_positions.extend(a.iter().chain(b.iter()).chain(c.iter()).chain(d.iter()));
    }

    let result = potential_positions
        .iter()
        .filter(|p| {
            if p.x < 0 || p.x > limit || p.y < 0 || p.y > limit {
                return false;
            }

            return sensors
                .iter()
                .all(|Sensor { sensor, beacon }| distance(&sensor, &beacon) < distance(p, sensor));
        })
        .next()
        .map(|Point { x, y }| (x * 4000000) + y);

    println!("{:?}", result);
}

fn rotate(vec: &Vec<Point>) -> Vec<Point> {
    vec.iter()
        .map(|Point { x, y }| Point {
            x: -y,
            y: x.clone(),
        })
        .collect::<Vec<_>>()
}

fn distance(a: &Point, b: &Point) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
