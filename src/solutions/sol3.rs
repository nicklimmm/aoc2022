use std::collections::HashSet;

use crate::utils::read_input_lines;

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

pub fn solve_a() {
    let mut ans = 0;
    for line in read_input_lines("input3.txt") {
        let (first, second) = line.split_at(line.len() / 2);

        let first_set: HashSet<char> = HashSet::from_iter(first.chars());
        let second_set = HashSet::from_iter(second.chars());

        for &c in first_set.intersection(&second_set) {
            ans += priority(c)
        }
    }

    println!("{ans}");
}

pub fn solve_b() {
    let mut ans = 0;
    let mut group_set = HashSet::new();
    let mut i = 0;
    for line in read_input_lines("input3.txt") {
        if i == 0 {
            group_set.extend(line.chars());
        } else {
            let cur_set: HashSet<char> = HashSet::from_iter(line.chars());
            group_set.retain(|c| cur_set.contains(c));
        }

        i += 1;
        if i == 3 {
            for &c in group_set.iter() {
                ans += priority(c)
            }
            group_set.clear();
            i = 0;
        }
    }

    println!("{ans}");
}
