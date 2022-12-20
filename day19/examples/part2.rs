#![feature(generic_arg_infer)]
#![feature(bool_to_option)]

use parse_display::{Display, FromStr};
use rayon::prelude::*;
use std::io::{self};
use std::ops::{Add, Sub};

#[derive(PartialEq, Debug, Clone, Eq, Copy, Default)]
struct Store {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Add for Store {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl Sub for Store {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl Store {
    fn new() -> Self {
        Store {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

const ORE_ROBOT: Store = Store {
    ore: 1,
    clay: 0,
    obsidian: 0,
    geode: 0,
};

const CLAY_ROBOT: Store = Store {
    ore: 0,
    clay: 1,
    obsidian: 0,
    geode: 0,
};

const OBSIDIAN_ROBOT: Store = Store {
    ore: 0,
    clay: 0,
    obsidian: 1,
    geode: 0,
};

const GEODE_ROBOT: Store = Store {
    ore: 0,
    clay: 0,
    obsidian: 0,
    geode: 1,
};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq, Copy)]
#[display("Blueprint {n}: Each ore robot costs {ore_robot.ore} ore. Each clay robot costs {clay_robot.ore} ore. Each obsidian robot costs {obsidian_robot.ore} ore and {obsidian_robot.clay} clay. Each geode robot costs {geode_robot.ore} ore and {geode_robot.obsidian} obsidian.")]
struct Blueprint {
    n: usize,

    #[from_str(default)]
    ore_robot: Store,

    #[from_str(default)]
    clay_robot: Store,

    #[from_str(default)]
    obsidian_robot: Store,

    #[from_str(default)]
    geode_robot: Store,
}

const TIME_LIMIT: usize = 32;

fn main() {
    let blueprints = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    let result: usize = blueprints
        .par_iter()
        .take(3)
        .map(|b| {
            let robots = [b.ore_robot, b.clay_robot, b.obsidian_robot, b.geode_robot];
            let geodes = number_of_geodes(
                &b,
                Store::new(),
                ORE_ROBOT,
                TIME_LIMIT,
                Store {
                    ore: robots.iter().map(|r| r.ore).max().unwrap(),
                    clay: robots.iter().map(|r| r.clay).max().unwrap(),
                    obsidian: robots.iter().map(|r| r.obsidian).max().unwrap(),
                    geode: 0,
                },
            );
            geodes
        })
        .product();

    println!("{:?}", result);
}
fn number_of_geodes(
    blueprint: &Blueprint,
    bank: Store,
    robot_yield: Store,
    time: usize,
    equilibrium: Store,
) -> usize {
    let mut m = bank.geode;

    let no_more_ore = robot_yield.ore >= equilibrium.ore;
    let no_more_clay = robot_yield.clay >= equilibrium.clay;
    let no_more_obsidian = robot_yield.obsidian >= equilibrium.obsidian;

    if no_more_ore && no_more_clay && no_more_obsidian {
        return m.max((0.5 * (time as f32 - 1.0) * time as f32) as usize);
    }

    for (cost, robot, no_more) in [
        (blueprint.ore_robot, ORE_ROBOT, no_more_ore),
        (blueprint.clay_robot, CLAY_ROBOT, no_more_clay),
        (blueprint.obsidian_robot, OBSIDIAN_ROBOT, no_more_obsidian),
        (blueprint.geode_robot, GEODE_ROBOT, false),
    ] {
        if no_more {
            continue;
        }

        let t = afford_when(&cost, &bank, &robot_yield).unwrap_or(usize::MAX);

        if t <= time {
            let resources = Store {
                ore: t * robot_yield.ore,
                clay: t * robot_yield.clay,
                obsidian: t * robot_yield.obsidian,
                geode: t * robot_yield.geode,
            };
            m = m.max(number_of_geodes(
                blueprint,
                bank + resources - cost,
                robot_yield + robot,
                time - t,
                equilibrium,
            ))
        }
    }

    return m.max(bank.geode);
}

fn afford_when(cost: &Store, bank: &Store, robot_yield: &Store) -> Option<usize> {
    let ore_time = time_until(cost.ore, bank.ore, robot_yield.ore);
    let clay_time = time_until(cost.clay, bank.clay, robot_yield.clay);
    let obsidian_time = time_until(cost.obsidian, bank.obsidian, robot_yield.obsidian);

    let t = ore_time.max(clay_time).max(obsidian_time);

    (t.ceil() as usize).checked_add(1)
}

fn time_until(cost: usize, bank: usize, robot_yield: usize) -> f32 {
    if cost == 0 {
        return 0.0;
    }
    if robot_yield == 0 {
        return f32::MAX;
    }
    (cost as f32 - bank as f32) / robot_yield as f32
}
