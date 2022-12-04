use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input_lines(file_name: &str) -> impl Iterator<Item = String> {
    let f = File::open(String::from("./src/solutions/inputs/") + file_name).unwrap();
    BufReader::new(f).lines().map(|line| line.unwrap())
}
