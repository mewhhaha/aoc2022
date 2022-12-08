use std::{
    collections::{HashSet, VecDeque},
    io::{self, Read},
};

const FULL: usize = 14;

fn main() {
    let stdin = io::stdin();
    let answer = stdin
        .lock()
        .bytes()
        .flatten()
        .enumerate()
        .scan(VecDeque::new(), |acc, (i, c)| {
            push_queue(acc, c);

            if is_marker(acc) {
                None
            } else {
                Some(i)
            }
        })
        .last();

    if let Some(size) = answer {
        let offset_zero_index = 1;
        let offset_stop_index = 1;
        println!("{:?}", size + offset_zero_index + offset_stop_index);
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
