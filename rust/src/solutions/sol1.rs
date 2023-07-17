use crate::utils::read_input_lines;
use std::{cmp::max, collections::BinaryHeap};

pub fn solve_a() {
    let mut ans = 0;
    let mut cur = 0;
    for line in read_input_lines("input1.txt") {
        if line.is_empty() {
            ans = max(ans, cur);
            cur = 0;
        } else {
            cur += line.parse::<i32>().unwrap();
        }
    }

    println!("{}", max(ans, cur));
}

pub fn solve_b() {
    let mut heap = BinaryHeap::new();
    let mut cur = 0;
    for line in read_input_lines("input1.txt") {
        if line.is_empty() {
            heap.push(-cur);
            cur = 0;

            if heap.len() > 3 {
                heap.pop();
            }
        } else {
            cur += line.parse::<i32>().unwrap();
        }
    }

    heap.push(-cur);
    if heap.len() > 3 {
        heap.pop();
    }

    let mut ans = 0;
    while let Some(v) = heap.pop() {
        ans += -v;
    }

    println!("{ans}");
}
