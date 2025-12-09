use std::collections::HashMap;
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
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    //    day09::run(&read_lines("inputs/test09.txt"));
    day09::run(&read_lines("inputs/day09.txt"));
    day08::run(&read_lines("inputs/day08.txt"));
    day07::run(&read_lines("inputs/day07.txt"));
    day06::run(&read_to_string("inputs/day06.txt").unwrap());
    day05::run(&read_to_string("inputs/day05.txt").unwrap());
    day04::run(&read_lines("inputs/day04.txt"));
    day03::run(&read_lines("inputs/day03.txt"));
    day02::run(&read_to_string("inputs/day02.txt").unwrap());
    day01::run(&read_lines("inputs/day01.txt"));
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Ord, PartialOrd)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}
impl Coord {
    pub fn new(row: usize, col: usize) -> Coord {
        Coord {
            row: row as i32,
            col: col as i32,
        }
    }

    pub fn add(&self, coord: &Coord, times: i32) -> Coord {
        Coord {
            row: self.row + (coord.row * times),
            col: self.col + (coord.col * times),
        }
    }

    pub fn distance(&self, other: &Coord) -> usize {
        ((self.row - other.row).abs() + (self.col - other.col).abs()) as usize
    }

    pub fn directions() -> Vec<Coord> {
        let mut dirs: Vec<Coord> = vec![];
        for row in [-1, 0, 1] {
            for col in [-1, 0, 1] {
                if !(row == 0 && col == 0) {
                    dirs.push(Coord { row, col });
                }
            }
        }
        dirs
    }

    pub fn neighbors(&self) -> Vec<Coord> {
        Coord::directions()
            .iter()
            .map(|c| Coord {
                row: self.row + c.row,
                col: self.col + c.col,
            })
            .collect()
    }

    pub fn turn_left(&self) -> Coord {
        Coord {
            row: 0 - self.col,
            col: self.row,
        }
    }

    pub fn turn_right(&self) -> Coord {
        Coord {
            row: self.col,
            col: 0 - self.row,
        }
    }

    pub fn up(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col,
        }
    }
    pub fn down(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }
    pub fn left(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col - 1,
        }
    }
    pub fn right(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub coords: HashMap<Coord, String>,
    pub nrows: usize,
    pub ncols: usize,
}
impl Grid {
    pub fn print(&self) {
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                match self.coords.get(&Coord::new(row, col)) {
                    Some(x) => print!("{}", x),
                    None => print!(" "),
                }
            }
            println!();
        }
    }

    pub fn print_path(&self, path: &Vec<Coord>) {
        let mut grid = self.clone();
        let mut i = 0;
        for coord in path {
            i += 1;
            grid.coords
                .insert(coord.clone(), String::from((i % 10).to_string()));
        }
        grid.print();
    }

    pub fn from_lines(lines: &Vec<String>) -> Grid {
        let nrows = lines.len();
        let mut ncols = 0usize;

        let mut coords: HashMap<Coord, String> = HashMap::new();
        for row in 0..lines.len() {
            // chars will have "" as its first and last elements -- ignore
            let chars: Vec<&str> = lines[row].split("").collect();
            ncols = chars.len() - 2;
            for col in 0..ncols {
                coords.insert(Coord::new(row, col), chars[col + 1].to_string());
            }
        }

        Grid {
            coords,
            nrows,
            ncols,
        }
    }

    pub fn find(&self, str: &str) -> Option<Coord> {
        let string = String::from(str);
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                let coord = Coord::new(row, col);
                match self.coords.get(&coord) {
                    Some(s) => {
                        if s == &string {
                            return Some(coord);
                        }
                    }
                    _ => (),
                }
            }
        }
        None
    }

    pub fn all_coords(&self) -> Vec<Coord> {
        let mut all_coords: Vec<Coord> = vec![];
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                all_coords.push(Coord::new(row, col));
            }
        }
        all_coords
    }

    pub fn in_bounds(&self, coord: &Coord) -> bool {
        coord.row >= 0
            && coord.row < self.nrows as i32
            && coord.col >= 0
            && coord.col < self.ncols as i32
    }
}
