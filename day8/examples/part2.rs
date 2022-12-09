use std::io;

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

    let mut result = 0;

    let max_row = rows.len() - 1;
    let max_col = rows[0].len() - 1;

    for (y, row) in rows.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let is_edge = y == 0 || x == 0 || y == max_row || x == max_col;
            if is_edge {
                continue;
            }

            let row_score = calc_scenic_rows(x, y, &rows);
            let col_score = calc_scenic_rows(y, x, &columns);

            result = result.max(row_score * col_score);
        }
    }

    println!("{:?}", result);
}

fn calc_scenic_rows(x: usize, y: usize, rows: &Vec<Vec<i32>>) -> i32 {
    let (left, rest) = rows[y].split_at(x);
    let (first, right) = rest.split_first().unwrap();
    let l = calc_row(&first, left.len(), left.iter().rev());
    let r = calc_row(&first, right.len(), right.iter());

    return l * r;
}

fn calc_row<'a, I>(height: &i32, edge: usize, row: I) -> i32
where
    I: Iterator<Item = &'a i32>,
{
    let score = row.take_while(|x| *x < height).count() as i32;
    let over_edge = score == edge as i32;
    if over_edge {
        score
    } else {
        score + 1
    }
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
