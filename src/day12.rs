use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::Coord;
use crate::Grid;

use rayon::prelude::*;

pub fn run(input: &String) {
    let mut inputs: Vec<String> = input.trim().split("\n\n").map(String::from).collect();
    let region_str = inputs.pop().unwrap();

    let presents: Vec<Grid> = inputs
        .iter()
        .map(|input| {
            let (_header, rest) = input.split_once("\n").unwrap();
            Grid::from_lines(&rest.split("\n").map(String::from).collect())
        })
        .collect();
    let regions: Vec<Region> = region_str
        .split("\n")
        .map(|line| Region::from(line))
        .collect();

    let fit_count: usize = regions.par_iter().filter(|r| r.can_fit(&presents)).count();
    println!("day 12 part 1: {}", fit_count);
}

#[derive(Clone, PartialEq, Eq)]
struct Region {
    grid: Grid,
    present_counts: Vec<usize>,
    rowmax: usize,
    colmax: usize,
    presents_placed: usize,
}
impl From<&str> for Region {
    fn from(line: &str) -> Self {
        let (dimensions, presents_str) = line.split_once(": ").unwrap();
        let (colstr, rowstr) = dimensions.split_once("x").unwrap();

        let grid = Grid {
            coords: HashMap::new(),
            nrows: rowstr.parse().unwrap(),
            ncols: colstr.parse().unwrap(),
        };
        let present_counts: Vec<usize> = presents_str
            .split(" ")
            .flat_map(|s| s.parse::<usize>())
            .collect();

        Self {
            grid: grid,
            present_counts: present_counts,
            rowmax: 0,
            colmax: 0,
            presents_placed: 0,
        }
    }
}
impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.presents_placed.cmp(&other.presents_placed) {
            Ordering::Equal => self.density().total_cmp(&other.density()),
            x => x,
        }
    }
}
impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Region {
    pub fn density(&self) -> f64 {
        let mut count = 0usize;
        for row in 0..(self.rowmax as i32) {
            for col in 0..(self.colmax as i32) {
                match self.grid.coords.get(&Coord { row: row, col: col }) {
                    None => (),
                    Some(s) => match s.as_ref() {
                        "#" => {
                            count += 1;
                        }
                        _ => (),
                    },
                }
            }
        }
        count as f64 / (self.rowmax * self.colmax) as f64
    }

    pub fn can_fit(&self, presents: &Vec<Grid>) -> bool {
        let mut heap = BinaryHeap::new();
        heap.push(self.clone());

        let mut placed_so_far = 0;
        while let Some(region) = heap.pop() {
            for (i, present) in presents.iter().enumerate() {
                if region.present_counts[i] == 0 {
                    continue;
                }

                // if 2 seems magic here, it is
                if placed_so_far > 2 + region.presents_placed {
                    return false;
                }
                placed_so_far = region.presents_placed;
                'perm: for perm in present.permutations() {
                    for row in 0..region.grid.nrows {
                        for col in 0..region.grid.ncols {
                            let coord = Coord {
                                row: row as i32,
                                col: col as i32,
                            };
                            if region.grid.can_place(&perm, &coord) {
                                let mut present_counts = region.present_counts.clone();
                                present_counts[i] -= 1;

                                let mut grid = region.grid.clone();
                                grid.place(&perm, &coord);
                                if present_counts.iter().sum::<usize>() == 0 {
                                    return true;
                                }

                                let rowmax =
                                    cmp::max(region.rowmax, coord.row as usize + perm.nrows);
                                let colmax =
                                    cmp::max(region.colmax, coord.col as usize + perm.ncols);
                                let region = Region {
                                    grid,
                                    present_counts,
                                    rowmax,
                                    colmax,
                                    presents_placed: region.presents_placed + 1,
                                };
                                heap.push(region);
                                continue 'perm;
                            }
                        }
                    }
                }
            }
        }

        false
    }
}
