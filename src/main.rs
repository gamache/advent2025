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
mod day03;

fn main() {
    day03::run(&read_lines("inputs/day03.txt"));
    day02::run(&read_to_string("inputs/day02.txt").unwrap());
    day01::run(&read_lines("inputs/day01.txt"));
}
