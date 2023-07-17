use std::fmt::Display;

use crate::utils::read_input_lines;

fn valid_sample(cycle: i32) -> bool {
    matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220)
}

pub fn solve_a() {
    let mut cycle = 1;
    let mut register_x = 1;
    let mut total_signal_strength = 0;
    for line in read_input_lines("input10.txt") {
        let args: Vec<&str> = line.split_whitespace().collect();
        match args[0] {
            "noop" => {
                if valid_sample(cycle) {
                    total_signal_strength += cycle * register_x;
                }
                cycle += 1;
            }
            "addx" => {
                for _ in 0..2 {
                    if valid_sample(cycle) {
                        total_signal_strength += cycle * register_x;
                    }
                    cycle += 1;
                }
                register_x += args[1].parse::<i32>().unwrap();
            }
            _ => panic!("Invalid argument"),
        }
    }

    println!("{total_signal_strength}");
}

struct Screen {
    sprite_pos: i32,
    crt_pos: usize,
    pixels: Vec<char>,
}

impl Screen {
    fn new() -> Self {
        Screen {
            sprite_pos: 1,
            crt_pos: 0,
            pixels: vec!['.'; 240],
        }
    }

    fn draw(&mut self) {
        let row_pos = (self.crt_pos % 40) as i32;
        self.pixels[self.crt_pos] =
            if self.sprite_pos - 1 <= row_pos && row_pos <= self.sprite_pos + 1 {
                '#'
            } else {
                '.'
            }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self.pixels.chunks(40).map(String::from_iter).collect();
        write!(f, "{}", &lines.join("\n"))
    }
}

pub fn solve_b() {
    let mut screen = Screen::new();
    for line in read_input_lines("input10.txt") {
        let args: Vec<&str> = line.split_whitespace().collect();
        match args[0] {
            "noop" => {
                screen.draw();
                screen.crt_pos += 1;
            }
            "addx" => {
                for _ in 0..2 {
                    screen.draw();
                    screen.crt_pos += 1;
                }
                screen.sprite_pos += args[1].parse::<i32>().unwrap();
            }
            _ => panic!("Invalid argument"),
        }
    }

    println!("{screen}");
}
