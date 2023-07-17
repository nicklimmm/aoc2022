use std::{array, collections::VecDeque};

use crate::utils::read_input_lines;

struct MoveInput {
    amount: usize,
    from: usize,
    to: usize,
}

fn extract_input() -> ([Vec<char>; 9], Vec<MoveInput>) {
    let mut input_lines_iter = read_input_lines("input5.txt");
    let mut stacks: [Vec<char>; 9] = array::from_fn(|_| vec![]);
    for _ in 0..8 {
        let line = input_lines_iter.next().unwrap();
        let line = line.as_bytes();
        for (i, stack) in stacks.iter_mut().enumerate() {
            let j = 1 + 4 * i;
            if line[j] == b' ' {
                continue;
            }
            stack.push(line[j] as char);
        }
    }

    stacks.iter_mut().for_each(|stack| stack.reverse());

    input_lines_iter.next();
    input_lines_iter.next();

    let move_inputs = input_lines_iter
        .map(|line| {
            let x: Vec<_> = line.split_whitespace().collect();
            MoveInput {
                amount: x[1].parse::<usize>().unwrap(),
                from: x[3].parse::<usize>().unwrap(),
                to: x[5].parse::<usize>().unwrap(),
            }
        })
        .collect();

    (stacks, move_inputs)
}

fn make_move(stacks: &mut [Vec<char>; 9], move_input: &MoveInput, in_order: bool) {
    let mut temp = VecDeque::new();
    for _ in 0..move_input.amount {
        let val = stacks[move_input.from - 1].pop().unwrap();
        if in_order {
            temp.push_back(val);
        } else {
            temp.push_front(val);
        }
    }

    while let Some(val) = temp.pop_back() {
        stacks[move_input.to - 1].push(val);
    }
}

pub fn solve_a() {
    let (mut stacks, move_inputs) = extract_input();
    move_inputs.iter().for_each(|move_input| {
        make_move(&mut stacks, move_input, false);
    });

    stacks.iter().for_each(|stack| {
        print!("{}", stack.last().unwrap());
    });
    println!();
}

pub fn solve_b() {
    let (mut stacks, move_inputs) = extract_input();
    move_inputs.iter().for_each(|move_input| {
        make_move(&mut stacks, move_input, true);
    });

    stacks.iter().for_each(|stack| {
        print!("{}", stack.last().unwrap());
    });
    println!();
}
