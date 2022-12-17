#![feature(slice_group_by)]

use parse_display::FromStr;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io;

#[derive(FromStr, PartialEq, Debug, Clone, Eq, Hash)]
#[from_str(
    regex = "Valve (?P<name>[A-Z]{2}) has flow rate=(?P<rate>[0-9]+); tunnels? leads? to valves? (?P<tunnels>.*)"
)]
struct Valve {
    name: String,
    rate: i32,
    tunnels: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Edge {
    cost: i32,
    position: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Graph = Vec<Vec<Edge>>;

const TIME_LIMIT: i32 = 26;

fn main() {
    let valves = io::stdin()
        .lines()
        .filter_map(|line| line.unwrap().parse::<Valve>().ok())
        .collect::<Vec<_>>();

    let reverse_mapping = valves
        .iter()
        .enumerate()
        .map(|(i, x)| (&x.name, i))
        .collect::<HashMap<_, _>>();

    let graph = valves
        .iter()
        .map(|Valve { tunnels, .. }| {
            tunnels
                .split(", ")
                .map(|tunnel| Edge {
                    cost: 1,
                    position: *reverse_mapping.get(&tunnel.to_string()).unwrap(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut valves_with_rate = vec![*reverse_mapping.get(&"AA".to_string()).unwrap()];
    valves_with_rate.extend(valves.iter().enumerate().filter_map(|(i, g)| {
        if g.rate > 0 {
            Some(i)
        } else {
            None
        }
    }));

    let tiny_graph = valves_with_rate
        .iter()
        .map(|start| {
            valves_with_rate
                .iter()
                .enumerate()
                .filter_map(|(i, end)| {
                    if start == end {
                        return None;
                    }
                    djikstra(&graph, *start, *end).map(|cost| Edge {
                        cost: cost + 1, // +1 for turning it on
                        position: i,
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rates = valves_with_rate
        .iter()
        .map(|i| valves[*i].rate)
        .collect::<Vec<_>>();

    let split_factor = tiny_graph.len() / 2 + 1;
    let result = max_flow(&tiny_graph, &rates, 1, &tiny_graph[0], split_factor);

    println!("{:?}", result);
}

fn max_flow(
    graph: &Vec<Vec<Edge>>,
    rates: &Vec<i32>,
    time: i32,
    next: &Vec<Edge>,
    split: usize,
) -> i32 {
    let mut m = 0;
    for edge in next.iter() {
        let next_time = time + edge.cost;
        if next_time > TIME_LIMIT {
            continue;
        }

        let this_value = (TIME_LIMIT - next_time + 1) * rates[edge.position];

        let without_self = keep_positions(graph[edge.position].clone(), next);
        let that_value = max_flow(graph, rates, next_time, &without_self, split);

        m = m.max(this_value + that_value);

        if without_self.len() < split {
            let start = keep_positions(graph[0].clone(), &without_self);

            let split_value = max_flow(graph, rates, 1, &start, 0);
            m = m.max(this_value + split_value);
        }
    }

    return m;
}

fn keep_positions(edges: Vec<Edge>, positions: &Vec<Edge>) -> Vec<Edge> {
    edges
        .into_iter()
        .filter(|x| positions.iter().any(|n| x.position == n.position))
        .collect::<Vec<_>>()
}

fn djikstra(graph: &Graph, start: usize, end: usize) -> Option<i32> {
    let mut visited = vec![i32::MAX; graph.len()];
    let mut remaining = BinaryHeap::<State>::new();

    remaining.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = remaining.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > visited[position] {
            continue;
        }

        for edge in &graph[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.position,
            };

            if next.cost < visited[next.position] {
                remaining.push(next);
                visited[next.position] = next.cost;
            }
        }
    }

    return None;
}
