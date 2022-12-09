use core::panic;
use std::{collections::HashSet, ops::Sub};

use crate::utils::read_input_lines;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn step(&mut self, dir: &Direction) -> &mut Self {
        match *dir {
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
        }

        self
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<Position> for (i32, i32) {
    fn from(p: Position) -> Self {
        (p.x, p.y)
    }
}

#[derive(Debug)]
struct RopeState {
    knots: Vec<Position>,
}

impl RopeState {
    fn new(len: usize) -> Self {
        RopeState {
            knots: vec![Position::new(0, 0); len],
        }
    }

    fn step(&mut self, dir: &Direction) {
        self.knots[0].step(dir);
        for i in 1..self.knots.len() {
            self.fix_body(i);
        }
    }

    fn fix_body(&mut self, i: usize) {
        let Position { x: dx, y: dy } = self.knots[i - 1].clone() - self.knots[i].clone();
        match (dx, dy) {
            (2, 0) => {
                self.knots[i].x += 1;
            }
            (2, 1) | (2, 2) | (1, 2) => {
                self.knots[i].x += 1;
                self.knots[i].y += 1;
            }
            (0, 2) => {
                self.knots[i].y += 1;
            }
            (-2, 1) | (-2, 2) | (-1, 2) => {
                self.knots[i].x -= 1;
                self.knots[i].y += 1;
            }
            (-2, 0) => {
                self.knots[i].x -= 1;
            }
            (-2, -1) | (-2, -2) | (-1, -2) => {
                self.knots[i].x -= 1;
                self.knots[i].y -= 1;
            }
            (0, -2) => {
                self.knots[i].y -= 1;
            }
            (1, -2) | (2, -2) | (2, -1) => {
                self.knots[i].x += 1;
                self.knots[i].y -= 1;
            }
            _ => {}
        }
    }
}

pub fn solve_a() {
    let mut state = RopeState::new(2);
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    // starting
    tail_visited.insert((0, 0));

    for line in read_input_lines("input9.txt") {
        let args: Vec<&str> = line.split_whitespace().collect();
        let dir: Direction = args[0].into();
        for _ in 0..args[1].parse::<u32>().unwrap() {
            state.step(&dir);
            tail_visited.insert(state.knots.last().unwrap().clone().into());
        }
    }

    println!("{}", tail_visited.len());
}

pub fn solve_b() {
    let mut state = RopeState::new(10);
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    // starting
    tail_visited.insert((0, 0));

    for line in read_input_lines("input9.txt") {
        let args: Vec<&str> = line.split_whitespace().collect();
        let dir: Direction = args[0].into();
        for _ in 0..args[1].parse::<u32>().unwrap() {
            state.step(&dir);
            tail_visited.insert(state.knots.last().unwrap().clone().into());
        }
    }

    println!("{}", tail_visited.len());
}
