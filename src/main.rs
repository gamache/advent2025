use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

mod day01;
mod day02;

fn main() {
    day02::run(&read_to_string("inputs/test02.txt").unwrap());
    day02::run(&read_to_string("inputs/day02.txt").unwrap());
    day01::run(&read_lines("inputs/day01.txt"));
}
