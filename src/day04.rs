use crate::Coord;
use crate::Grid;

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);
    part1(&grid);
    part2(&grid);
}

fn accessible_coords(grid: &Grid) -> Vec<Coord> {
    grid.all_coords()
        .iter()
        .filter(|&coord| grid.coords.get(coord) == Some(&String::from("@")))
        .filter(|&coord| count_rolls(grid, coord) < 4)
        .map(|&coord| coord.clone())
        .collect()
}

fn count_rolls(grid: &Grid, coord: &Coord) -> usize {
    let neighbors = coord.neighbors();
    neighbors
        .iter()
        .filter(|coord| grid.coords.get(coord) == Some(&String::from("@")))
        .count()
}

fn part1(grid: &Grid) {
    let count = accessible_coords(grid).len();
    println!("day 04 part 1: {}", count);
}

fn part2(grid: &Grid) {
    let mut g = grid.clone();
    let mut removed = 0;

    loop {
        let coords = accessible_coords(&g);
        if coords.len() == 0 {
            break;
        }
        for coord in coords {
            g.coords.insert(coord, String::from("x"));
            removed += 1;
        }
    }

    println!("day 04 part 2: {}", removed);
}
