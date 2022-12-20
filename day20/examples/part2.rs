use std::io::{self};

const DECRYPTION_KEY: i64 = 811589153;

fn main() {
    let numbers = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.parse::<i64>().unwrap() * DECRYPTION_KEY)
        .enumerate()
        .collect::<Vec<_>>();

    let mut file = numbers.clone();
    let length = numbers.len();

    for _ in 0..10 {
        for (i, number) in numbers.iter() {
            if length == 7 {
                println!("{:?}", file.iter().map(|x| x.1).collect::<Vec<_>>());
            }

            let from = file.iter().position(|n| n.0 == *i).unwrap();

            file.remove(from);
            if number < &0 {
                file.rotate_right((number.abs() as usize) % (length - 1));
            } else {
                file.rotate_left(*number as usize % (length - 1));
            }

            file.insert(from, (*i, *number));

            if length == 7 {
                println!("{:?}", file.iter().map(|x| x.1).collect::<Vec<_>>());
            }
        }
    }

    let zero_position = file.iter().position(|(_, x)| x == &0).unwrap();

    let result: i64 = [1000, 2000, 3000]
        .into_iter()
        .map(|i| file[(zero_position + i) % file.len()].1)
        .sum();

    println!("{:?}", result);
}
