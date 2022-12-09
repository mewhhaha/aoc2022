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

    memo.extend(check_rows(&rows).into_iter());
    memo.extend(
        check_rows(&transpose(&rows))
            .into_iter()
            .map(|(x, y)| (y, x)),
    );

    println!("{:?}", memo.len());
}

fn check_rows(grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for coord in check_row(y, row.iter()) {
            coords.push(coord);
        }

        let l = row.len() - 1;
        for (x, y) in check_row(y, row.iter().rev()) {
            coords.push((l - x, y));
        }
    }

    return coords;
}

fn check_row<'a>(
    y: usize,
    row: impl Iterator<Item = &'a i32>,
) -> impl Iterator<Item = (usize, usize)> {
    row.scan(-1, is_visible)
        .enumerate()
        .filter_map(move |(x, visible)| if visible { Some((x, y)) } else { None })
}

fn is_visible(highest: &mut i32, i: &i32) -> Option<bool> {
    if i > highest {
        *highest = *i;
        return Some(true);
    }

    return Some(false);
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
