use std::collections::{HashSet, VecDeque};

use crate::utils::read_input_lines;

pub fn solve_a() {
    let input = read_input_lines("input6.txt").next().unwrap();
    let mut s = HashSet::new();
    let mut q = VecDeque::new();
    let k = 4;

    for (i, c) in input.chars().enumerate() {
        while s.contains(&c) {
            s.remove(&q.pop_front().unwrap());
        }

        s.insert(c);
        q.push_back(c);

        if q.len() == k {
            println!("{}", i + 1);
            return;
        }
    }
}

pub fn solve_b() {
    let input = read_input_lines("input6.txt").next().unwrap();
    let mut s = HashSet::new();
    let mut q = VecDeque::new();
    let k = 14;

    for (i, c) in input.chars().enumerate() {
        while s.contains(&c) {
            s.remove(&q.pop_front().unwrap());
        }

        s.insert(c);
        q.push_back(c);

        if q.len() == k {
            println!("{}", i + 1);
            return;
        }
    }
}
