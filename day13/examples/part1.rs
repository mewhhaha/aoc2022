use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read};
use std::ops::Add;

fn main() {
    let mut text = String::new();
    _ = io::stdin().lock().read_to_string(&mut text);

    let pairs = text.split("\n\n").filter_map(|t| t.split_once("\n"));

    for pair in pairs {
        println!("{:?}", pair);
    }
}
