use crate::Coord;
use crate::Grid;

use rayon::prelude::*;

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);
    part1(&grid);
}

fn part1(grid: &Grid) {
    let count = grid
        .all_coords()
        .iter()
        .filter(|&coord| grid.coords.get(coord) == Some(&String::from("@")))
        .filter(|&coord| count_rolls(grid, coord) < 4)
        .count();
    println!("day 04 part 1: {}", count);
}

fn count_rolls(grid: &Grid, coord: &Coord) -> usize {
    let neighbors = coord.neighbors();
    neighbors.iter().filter(|coord| grid.coords.get(coord) == Some(&String::from("@"))).count()
}
