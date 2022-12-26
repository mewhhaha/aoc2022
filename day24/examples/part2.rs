use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
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

const ADJACENT: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
];

fn main() {
    let mut map = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Cut off walls
    map = map[1..map.len() - 1].to_vec();
    map = map
        .into_iter()
        .map(|line| line[1..line.len() - 1].to_vec())
        .collect();

    let number_of_rows = map.len() as i32;
    let number_of_columns = map[0].len() as i32;

    let max_time = lcm(number_of_columns, number_of_rows);

    let mut time_map = vec![vec![vec![]; map[0].len()]; map.len()];

    for p in coordinates(&map) {
        // A lot of duplicate work here since all columns and rows are the same, just offset by time
        time_map[p.y as usize][p.x as usize] = (0..max_time)
            .map(|t| {
                row_col(&map, &p).any(|tile| {
                    let hit = match map[tile.y as usize][tile.x as usize] {
                        '>' => Some(Point {
                            x: (tile.x as i32 + t).rem_euclid(number_of_columns),
                            y: tile.y,
                        }),
                        '<' => Some(Point {
                            x: (tile.x as i32 - t).rem_euclid(number_of_columns),
                            y: tile.y,
                        }),
                        '^' => Some(Point {
                            x: tile.x,
                            y: (tile.y as i32 - t).rem_euclid(number_of_rows),
                        }),
                        'v' => Some(Point {
                            x: tile.x,
                            y: (tile.y as i32 + t).rem_euclid(number_of_rows),
                        }),
                        _ => None,
                    };
                    Some(p) == hit
                })
            })
            .collect();
    }

    let edges = |(elapsed, position)| {
        let get_range = |t: usize, p: Point| {
            time_map
                .get(p.y as usize)
                .and_then(|v| v.get(p.x as usize))
                .map(|r| r.iter().cycle().skip(t).take(max_time as usize))
        };

        let max_wait = get_range(elapsed as usize, position)
            .and_then(|mut r| r.position(|p| *p).map(|i| i as i32))
            .unwrap_or(max_time);

        let edges = ADJACENT
            .iter()
            .map(|p| *p + position)
            .filter_map(|p| get_range(elapsed as usize + 1, p).map(|range| (p, range)))
            .flat_map(|(p, range)| {
                let mut possibilities = vec![];
                let mut group = false;
                let mut time = 1;
                for is_blocked in range {
                    if time > max_wait {
                        break;
                    };
                    if *is_blocked && group {
                        group = false;
                    } else if !is_blocked && !group {
                        possibilities.push((time, ((time + elapsed) % max_time, p)));
                        group = true;
                    }
                    time += 1;
                }

                possibilities
            })
            .collect::<Vec<_>>();

        edges
    };

    let start = Point { x: 0, y: -1 };
    let end = Point {
        x: number_of_columns - 1,
        y: number_of_rows,
    };
    let next_to_start = Point { x: 0, y: 0 };
    let next_to_end = Point {
        x: number_of_columns - 1,
        y: number_of_rows - 1,
    };

    let first = 1 + djikstra((0, start), |(_, p)| *p == next_to_end, edges).unwrap();

    let second = 1 + djikstra((first, end), |(_, p)| *p == next_to_start, edges).unwrap();

    let third = 1 + djikstra((first + second, start), |(_, p)| *p == next_to_end, edges).unwrap();

    let result = first + second + third;

    println!("{:?}", result);
}

fn row_col<'a>(map: &Vec<Vec<char>>, p: &'a Point) -> impl Iterator<Item = Point> + 'a {
    let y_length = map.len();
    let x_length = map[0].len();

    let column = (0..y_length).map(move |y| Point {
        x: p.x,
        y: y as i32,
    });
    let row = (0..x_length).map(move |x| Point {
        x: x as i32,
        y: p.y,
    });
    column.chain(row)
}

fn coordinates(map: &Vec<Vec<char>>) -> impl Iterator<Item = Point> {
    let y_length = map.len();
    let x_length = map[0].len();
    (0..y_length).flat_map(move |y| {
        (0..x_length).map(move |x| Point {
            x: x as i32,
            y: y as i32,
        })
    })
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<T: Ord> {
    cost: i32,
    position: T,
}

impl<T: Ord> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl<T: Ord> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn djikstra<T>(
    start: T,
    is_end: impl Fn(&T) -> bool,
    graph: impl Fn(T) -> Vec<(i32, T)>,
) -> Option<i32>
where
    T: std::hash::Hash,
    T: Ord,
    T: Eq,
    T: Clone,
{
    let mut visited = HashMap::<T, i32>::new();
    let mut remaining = BinaryHeap::<State<T>>::new();

    remaining.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = remaining.pop() {
        if is_end(&position) {
            return Some(cost);
        }

        if &cost > visited.get(&position).unwrap_or(&i32::MAX) {
            continue;
        }

        for (edge_cost, edge_position) in graph(position) {
            let next = State {
                cost: cost + edge_cost,
                position: edge_position,
            };

            if &next.cost < visited.get(&next.position).unwrap_or(&i32::MAX) {
                visited.insert(next.position.clone(), next.cost.clone());
                remaining.push(next);
            }
        }
    }

    return None;
}

// Copied from https://www.hackertouch.com/lowest-common-multiple-in-rust.html

fn lcm(first: i32, second: i32) -> i32 {
    first * second / gcd(first, second)
}

fn gcd(first: i32, second: i32) -> i32 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
