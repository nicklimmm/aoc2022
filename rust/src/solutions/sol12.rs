use std::collections::VecDeque;

use crate::utils::read_input_lines;

struct Input {
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<i32>>,
}

impl Input {
    fn new() -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut grid = vec![];
        for (i, line) in read_input_lines("input12.txt").enumerate() {
            if grid.len() == i {
                grid.push(vec![]);
            }
            for (j, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = (i, j);
                        grid[i].push(0);
                    }
                    'E' => {
                        end = (i, j);
                        grid[i].push(25);
                    }
                    c => {
                        grid[i].push((c as u8 - b'a') as i32);
                    }
                }
            }
        }

        Self { start, end, grid }
    }
}

const DIR_R: [i32; 4] = [0, 1, 0, -1];
const DIR_C: [i32; 4] = [1, 0, -1, 0];

fn bfs(input: Input, part_b: bool) -> u32 {
    let mut q = VecDeque::new();
    let (m, n) = (input.grid.len(), input.grid[0].len());
    let mut visited = vec![vec![false; n]; m];

    struct Item {
        pos: (usize, usize),
        dist: u32,
    }

    q.push_back(Item {
        pos: if !part_b { input.start } else { input.end },
        dist: 0,
    });

    while let Some(Item { pos, dist }) = q.pop_front() {
        if !part_b && pos == input.end || part_b && input.grid[pos.0][pos.1] == 0 {
            return dist;
        }

        if visited[pos.0][pos.1] {
            continue;
        }

        visited[pos.0][pos.1] = true;

        for k in 0..4 {
            let new_pos = (pos.0 as i32 + DIR_R[k], pos.1 as i32 + DIR_C[k]);

            // Out of bounds
            if new_pos.0 < 0 || new_pos.0 >= m as i32 || new_pos.1 < 0 || new_pos.1 >= n as i32 {
                continue;
            }

            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

            // Visited or jump too high
            let h_diff = input.grid[new_pos.0][new_pos.1] - input.grid[pos.0][pos.1];
            if visited[new_pos.0][new_pos.1] || (!part_b && h_diff > 1) || (part_b && h_diff < -1) {
                continue;
            }

            q.push_back(Item {
                pos: new_pos,
                dist: dist + 1,
            });
        }
    }

    0
}

pub fn solve_a() {
    let input = Input::new();
    println!("{}", bfs(input, false));
}

pub fn solve_b() {
    let input = Input::new();
    println!("{}", bfs(input, true));
}
