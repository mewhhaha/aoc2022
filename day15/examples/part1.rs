use parse_display::{Display, FromStr};
use std::collections::HashSet;
use std::io;
use std::ops::{Add, Sub};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[display("x={x}, y={y}")]
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
    let row_y = 2000000;

    let mut no_beacon = HashSet::<Point>::new();

    for Sensor { sensor, beacon } in sensors.iter() {
        let d = distance(&sensor, &beacon);

        for x in (sensor.x - d)..=(sensor.x + d) {
            let p = Point { x, y: row_y };
            if distance(&sensor, &p) <= d {
                no_beacon.insert(p);
            }
        }
    }

    for Sensor { beacon, .. } in sensors.iter() {
        no_beacon.remove(beacon);
    }

    println!("{:?}", no_beacon.len());
}

fn distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
