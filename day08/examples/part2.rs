use std::{convert::identity, io};

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

    let columns = transpose(&rows);

    let result = (0..rows.len())
        .flat_map(|y| (0..rows[0].len()).map(move |x| (x, y)))
        .map(|(x, y)| {
            let row_score = calc_scenic_rows(x, y, &rows);
            let col_score = calc_scenic_rows(y, x, &columns);

            row_score * col_score
        })
        .max_by_key(|x| x.clone())
        .unwrap_or(0);

    println!("{:?}", result);
}

fn calc_scenic_rows(x: usize, y: usize, rows: &Vec<Vec<i32>>) -> i32 {
    let row = &rows[y];
    let tree_house = &row[x];
    let left = row.iter().take(x).rev();
    let right = row.iter().skip(x + 1);

    let l = calc_scenic_row(&tree_house, left);
    let r = calc_scenic_row(&tree_house, right);

    l * r
}

fn calc_scenic_row<'a>(height: &i32, mut row: impl Iterator<Item = &'a i32>) -> i32 {
    row.try_fold(0, |count, x| {
        let next = count + 1;
        if x < height {
            Ok(next)
        } else {
            Err(next)
        }
    })
    .unwrap_or_else(identity)
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
