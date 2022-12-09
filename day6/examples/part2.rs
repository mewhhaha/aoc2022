use std::{
    collections::{HashSet, VecDeque},
    io::{self, Read},
};

const FULL: usize = 14;

fn main() {
    let stdin = io::stdin();
    let mut acc = VecDeque::new();

    let answer = stdin.lock().bytes().position(|c| {
        push_queue(&mut acc, c.unwrap());
        is_marker(&acc)
    });

    if let Some(size) = answer {
        let offset_zero_index = 1; // +1 for the zero index
        println!("{:?}", size + offset_zero_index);
    }
}

fn push_queue(q: &mut VecDeque<u8>, c: u8) {
    if q.len() == FULL {
        q.pop_back();
    }
    q.push_front(c);
}

fn is_marker(q: &VecDeque<u8>) -> bool {
    if q.len() != FULL {
        return false;
    }

    let set = q.iter().collect::<HashSet<_>>();

    return set.len() == q.len();
}
