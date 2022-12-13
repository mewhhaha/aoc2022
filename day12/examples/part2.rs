use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self};
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cost {
    value: i32,
    point: Point,
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

type Graph = HashMap<Point, u8>;

fn main() {
    let mut end = Point { x: 0, y: 0 };
    let graph = io::stdin()
        .lines()
        .flatten()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, i)| {
                    let p = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    match i as char {
                        'E' => {
                            end = p.clone();
                            (p, 'z' as u8)
                        }
                        'S' => (p, 'a' as u8),
                        _ => (p, i),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Graph>();

    let starts = graph.iter().filter_map(|(p, i)| {
        if *i == 'a' as u8 {
            Some(p.clone())
        } else {
            None
        }
    });

    let shortest = starts
        .filter_map(|start| djikstra(&graph, start, end))
        .min();

    println!("{:?}", shortest);
}

fn djikstra(graph: &Graph, start: Point, end: Point) -> Option<i32> {
    let mut visited = graph
        .keys()
        .map(|p| (p.clone(), i32::MAX))
        .collect::<HashMap<_, _>>();
    let mut remaining = BinaryHeap::<Cost>::new();

    remaining.push(Cost {
        value: 0,
        point: start,
    });

    while let Some(Cost { value, point }) = remaining.pop() {
        if point == end {
            return Some(value);
        }

        if Some(&value) > visited.get(&point) {
            continue;
        }

        for edge in edges(graph, point) {
            let next = Cost {
                value: value + 1,
                point: edge,
            };

            if Some(&next.value) < visited.get(&next.point) {
                remaining.push(next);
                visited.insert(next.point, next.value);
            }
        }
    }

    return None;
}

const ADJACENT: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: 0 },
];

fn edges(graph: &Graph, point: Point) -> impl Iterator<Item = Point> + '_ {
    let elevation = graph.get(&point).unwrap();

    ADJACENT.clone().into_iter().filter_map(move |p| {
        let edge = p + point;
        let adjacent_elevation = graph.get(&edge)?;
        if (*adjacent_elevation as i32 - *elevation as i32) <= 1 {
            Some(edge)
        } else {
            None
        }
    })
}
