#![feature(bool_to_option)]

use std::{collections::HashSet, io};

fn main() {
    let stdin = io::stdin();
    let rows = stdin
        .lines()
        .flatten()
        .map(|s| {
            s.chars()
                .filter_map(|x| x.to_digit(10).map(|x| x as i32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut memo = HashSet::new();

    memo.extend(check_rows(&rows));
    memo.extend(check_rows(&transpose(&rows)).map(flip));

    println!("{:?}", memo.len());
}

fn flip<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

fn check_rows(grid: &Vec<Vec<i32>>) -> impl Iterator<Item = (usize, usize)> + '_ {
    grid.iter().enumerate().flat_map(|(y, row)| {
        let check_row = |mut highest| {
            move |x| {
                if row[x] > highest {
                    highest = row[x];
                    Some((x, y))
                } else {
                    None
                }
            }
        };

        let left = (0..row.len()).filter_map(check_row(-1));
        let right = (0..row.len()).rev().filter_map(check_row(-1));

        left.chain(right)
    })
}

fn transpose<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = grid[0].len();
    let mut iters = grid.iter().map(|v| v.into_iter()).collect::<Vec<_>>();

    return (0..n)
        .map(|_| {
            iters
                .iter_mut()
                .map(|row| row.next().unwrap().clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
}
