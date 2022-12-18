#![feature(bool_to_option)]

use parse_display::FromStr;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

#[derive(Eq, PartialEq, Debug)]
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

const TIME_LIMIT: i32 = 30;

// I find the code I made quite hard to read,
// but basically just reduce the graph to just the starting position and the valves that have rates.
// This is done by calculating the shortest path between the relevant nodes, then just calculating
// the max flow that can be achieved using this much smaller graph.

fn main() {
    let valves = io::stdin()
        .lines()
        .filter_map(|line| line.unwrap().parse::<Valve>().ok())
        .collect::<Vec<_>>();

    let graph = valves
        .iter()
        .map(|Valve { tunnels, .. }| {
            tunnels
                .split(", ")
                .map(|tunnel| Edge {
                    cost: 1,
                    position: valves.iter().position(|v| v.name == tunnel).unwrap(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let valves_with_rate = vec![valves.iter().position(|v| v.name == "AA").unwrap()]
        .into_iter()
        .chain(
            valves
                .iter()
                .enumerate()
                .filter_map(|(i, g)| (g.rate > 0).then_some(i)),
        )
        .collect::<Vec<_>>();

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

    let result = max_flow(
        &tiny_graph,
        &rates,
        1,
        tiny_graph[0].iter().collect::<Vec<_>>(),
    );

    println!("{:?}", result);
}

fn max_flow(graph: &Vec<Vec<Edge>>, rates: &Vec<i32>, time: i32, next: Vec<&Edge>) -> i32 {
    let mut m = 0;
    for edge in next.iter() {
        let next_time = time + edge.cost;
        if next_time > TIME_LIMIT {
            continue;
        }

        let this_value = (TIME_LIMIT - next_time + 1) * rates[edge.position];

        let without_self = graph[edge.position]
            .iter()
            .filter(|e| next.iter().any(|n| n.position == e.position))
            .collect::<Vec<_>>();

        let that_value = max_flow(graph, rates, next_time, without_self);

        m = m.max(this_value + that_value);
    }

    return m;
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
