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
    let is_edge = |x, y| y == 0 || x == 0 || y == max_row || x == max_col;

    for (y, row) in rows.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_edge(x, y) {
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
    let l = calc_scenic_row(&first, left.iter().rev());
    let r = calc_scenic_row(&first, right.iter());

    return l * r;
}

fn calc_scenic_row<'a>(height: &i32, row: impl Iterator<Item = &'a i32>) -> i32 {
    let mut cancel = false; // cancel flag to include the element that stopped visibility
    row.take_while(|x| {
        if cancel {
            return false;
        }
        cancel = *x >= height;
        return true;
    })
    .count() as i32
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
