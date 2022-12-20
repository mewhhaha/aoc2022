use std::io::{self};
use std::ops::{Add, Sub};

fn main() {
    let numbers = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let result = 0;

    println!("{:?}", result);
}
