use crate::Coord;
use std::{cmp, collections::HashMap};

pub fn run(lines: &Vec<String>) {
    let coords: Vec<Coord> = lines
        .iter()
        .flat_map(|line| {
            let numbers: Vec<i32> = line.split(",").flat_map(|n| n.parse::<i32>()).collect();
            match numbers.len() {
                0 => None,
                _ => Some(Coord {
                    row: numbers[0],
                    col: numbers[1],
                }),
            }
        })
        .collect();
    part1(&coords);
    part2(&coords);
}

fn area(c1: &Coord, c2: &Coord) -> i64 {
    (1 + (c1.row - c2.row).abs() as i64) * (1 + (c1.col - c2.col).abs() as i64)
}

fn part1(reds: &Vec<Coord>) {
    let mut largest_area = 0i64;
    'outer: for r1 in reds {
        for r2 in reds {
            if r1 == r2 {
                continue 'outer;
            }
            let area = area(r1, r2);
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    println!("day 09 part 1: {}", largest_area);
}

fn part2(reds: &Vec<Coord>) {
    let mut floor = Floor::new(reds);
    let part2 = floor.largest_area();
    println!("day 09 part 2: {}", part2);
}

struct Floor {
    reds: Vec<Coord>,
    vert_edges: Vec<(Coord, Coord)>,
    rows_of_interest: Vec<i32>,
    cols_of_interest: Vec<i32>,
    inside_cache: HashMap<Coord, bool>,
}
impl Floor {
    pub fn new(reds: &Vec<Coord>) -> Self {
        let mut edges: Vec<(Coord, Coord)> = vec![];
        for i in 0..reds.len() {
            let r1 = reds[i];
            let r2 = reds[(i + 1) % reds.len()];
            edges.push((r1.clone(), r2.clone()));
        }

        let horiz_edges: Vec<(Coord, Coord)> = edges
            .iter()
            .filter(|(c1, c2)| c1.row == c2.row)
            .map(|edge| edge.clone())
            .collect();

        let vert_edges: Vec<(Coord, Coord)> = edges
            .iter()
            .filter(|(c1, c2)| c1.col == c2.col)
            .map(|edge| edge.clone())
            .collect();

        let mut rows_of_interest: Vec<i32> = horiz_edges
            .iter()
            .flat_map(|(c1, c2)| {
                [
                    c1.row - 1,
                    c1.row,
                    c1.row + 1,
                    c2.row - 1,
                    c2.row,
                    c2.row + 1,
                ]
            })
            .collect();
        rows_of_interest.sort();

        let mut cols_of_interest: Vec<i32> = horiz_edges
            .iter()
            .flat_map(|(c1, c2)| {
                [
                    c1.col - 1,
                    c1.col,
                    c1.col + 1,
                    c2.col - 1,
                    c2.col,
                    c2.col + 1,
                ]
            })
            .collect();
        cols_of_interest.sort();

        Self {
            reds: reds.clone(),
            vert_edges: vert_edges,
            cols_of_interest: cols_of_interest,
            rows_of_interest: rows_of_interest,
            inside_cache: HashMap::new(),
        }
    }

    fn is_inside(&mut self, c: &Coord) -> bool {
        // Draw a line from (0, c.col) to (c.row, c.col).
        // If it lands on a vertical edge, it's inside.
        // Otherwise, if it intersects an odd number of vertical edges, it's inside.
        match self.inside_cache.get(c) {
            Some(inside) => {
                return *inside;
            }
            None => {}
        }

        if self.on_vert_edge(c) {
            self.inside_cache.insert(c.clone(), true);
            return true;
        }
        let count = self
            .vert_edges
            .iter()
            .filter(|(c1, c2)| {
                let minrow = cmp::min(c1.row, c2.row);
                let maxrow = cmp::max(c1.row, c2.row);
                minrow <= c.row && c.row < maxrow && 0 <= c.col && c.col <= c1.col
            })
            .collect::<Vec<&(Coord, Coord)>>()
            .len();
        let inside = count % 2 == 1;
        self.inside_cache.insert(c.clone(), inside);
        inside
    }

    fn on_vert_edge(&self, c: &Coord) -> bool {
        for (c1, c2) in &self.vert_edges {
            if c1.col != c.col {
                continue;
            }
            let minrow = cmp::min(c1.row, c2.row);
            let maxrow = cmp::max(c1.row, c2.row);
            if minrow <= c.row && c.row <= maxrow {
                return true;
            }
        }
        false
    }

    fn box_is_inside(&mut self, c1: &Coord, c2: &Coord) -> bool {
        let minrow = cmp::min(c1.row, c2.row);
        let maxrow = cmp::max(c1.row, c2.row);
        let mincol = cmp::min(c1.col, c2.col);
        let maxcol = cmp::max(c1.col, c2.col);

        let mut row = minrow;

        while row <= maxrow {
            if row == minrow || self.rows_of_interest.contains(&row) {
                let mut col = mincol;

                while col <= maxcol {
                    if col == mincol || self.cols_of_interest.contains(&col) {
                        if !self.is_inside(&Coord { row: row, col: col }) {
                            return false;
                        }
                    }
                    match self.next_col(col) {
                        None => {
                            break;
                        }
                        Some(c) => {
                            col = c;
                        }
                    }
                }
            }
            match self.next_row(row) {
                None => {
                    break;
                }
                Some(r) => {
                    row = r;
                }
            }
        }

        true
    }

    fn next_col(&self, col: i32) -> Option<i32> {
        for c in &self.cols_of_interest {
            if *c > col {
                return Some(*c);
            }
        }
        None
    }

    fn next_row(&self, row: i32) -> Option<i32> {
        for r in &self.rows_of_interest {
            if *r > row {
                return Some(*r);
            }
        }
        None
    }

    pub fn largest_area(&mut self) -> i64 {
        let mut largest = 0i64;
        let reds = self.reds.clone();
        'outer: for r1 in &reds {
            for r2 in &reds {
                if r1 == r2 {
                    continue 'outer;
                }
                let area = area(r1, r2);
                if area > largest && self.box_is_inside(r1, r2) {
                    largest = area;
                }
            }
        }
        largest
    }
}
