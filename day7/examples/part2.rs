#![feature(map_try_insert)]

use parse_display::{Display, FromStr};
use std::{
    collections::HashMap,
    io::{self},
};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Command {
    #[display("$ ls")]
    List,

    #[display("$ cd /")]
    GoRoot,

    #[display("$ cd ..")]
    GoUp,

    #[display("$ cd {0}")]
    GoInto(String),
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Content {
    #[display("{0} {1}")]
    File(u32, String),

    #[display("dir {0}")]
    Directory(String),
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lines().flatten();

    let mut dirs = HashMap::<String, Vec<Content>>::new();

    let mut current_dir = Vec::<String>::new();

    for line in lines {
        if let Ok(command) = line.parse::<Command>() {
            match command {
                Command::GoInto(dir) => {
                    current_dir.push(dir);
                    let p = path(&current_dir);
                    _ = dirs.try_insert(p, Vec::new());
                }
                Command::GoUp => {
                    current_dir.pop();
                }
                _ => (),
            }
        } else if let Ok(file) = line.parse::<Content>() {
            let p = path(&current_dir);
            if let Some(dir) = dirs.get_mut(&p) {
                dir.push(file);
            }
        }
    }

    let mut sizes = HashMap::<String, i32>::new();

    for dir in dirs.keys() {
        if let None = sizes.get(dir) {
            _ = calc_directory(&dir, &mut sizes, &dirs)
        }
    }

    let total: i32 = sizes
        .iter()
        .filter_map(|(key, value)| if key.contains("/") { None } else { Some(value) })
        .sum();
    let minimum_size = total - 40000000;
    let smallest = sizes.into_values().filter(|x| *x >= minimum_size).min();
    println!("{:?}", smallest);
}

fn calc_directory(
    dir: &String,
    sizes: &mut HashMap<String, i32>,
    dirs: &HashMap<String, Vec<Content>>,
) -> i32 {
    if let Some(size) = sizes.get(dir) {
        return size.clone();
    }

    let mut size = 0;
    if let Some(content) = dirs.get(dir) {
        for c in content {
            match c {
                Content::File(s, _) => size += *s as i32,
                Content::Directory(name) => {
                    let p = path(&vec![dir.clone(), name.clone()]);
                    size += calc_directory(&p, sizes, dirs)
                }
            }
        }
    }

    sizes.insert(dir.clone(), size);

    return size;
}

fn path(ss: &Vec<String>) -> String {
    ss.join("/")
}
