use std::io;

fn main() {
    let base_10 = io::stdin().lines().flatten().map(from_snafu).sum::<i64>();

    let snafu = to_snafu(base_10 as u64);

    println!("{:?}", base_10);
    println!("{:?}", snafu);
    println!("{:?}", from_snafu(snafu));
}

fn from_snafu(s: String) -> i64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let position = 5_i64.pow(i as u32);

            position
                * match c {
                    '1' => 1,
                    '2' => 2,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => panic!(),
                }
        })
        .sum::<i64>()
}

fn to_snafu(s: u64) -> String {
    format_radix(s, 5)
        .chars()
        .rev()
        .fold("0".to_string(), |acc, c| {
            let carry_over = acc.chars().next().unwrap();

            match (carry_over, c) {
                ('0', '0') => "00",
                ('0', '1') => "01",
                ('0', '2') => "02",
                ('0', '3') => "1=",
                ('0', '4') => "1-",
                ('1', '0') => "01",
                ('1', '1') => "02",
                ('1', '2') => "1=",
                ('1', '3') => "1-",
                ('1', '4') => "10",
                _ => panic!(),
            }
            .to_string()
                + &acc[1..]
        })
        .chars()
        .skip_while(|c| c == &'0')
        .collect()
}

// Taken from https://stackoverflow.com/a/50278316

fn format_radix(mut x: u64, radix: u64) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m as u32, radix as u32).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
