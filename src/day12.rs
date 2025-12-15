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

    let fit_count: usize = regions.iter().filter(|r| r.can_fit(&presents)).count();
    println!("day 12 part 1: {}", fit_count);
}

#[derive(Clone, PartialEq, Eq)]
struct Region {
    grid: Grid,
    present_counts: Vec<usize>,
    rowmax: usize,
    colmax: usize,
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
        }
    }
}
impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering {
        let area = self.rowmax * self.colmax;
        let other_area = other.rowmax * other.colmax;
        other_area.cmp(&area)
    }
}
impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Region {
    pub fn can_fit(&self, presents: &Vec<Grid>) -> bool {
        let mut heap = BinaryHeap::new();
        heap.push(self.clone());

        while let Some(region) = heap.pop() {
            // println!("\nregion:");
            // region.grid.print();
            for (i, present) in presents.iter().enumerate() {
                if region.present_counts[i] == 0 {
                    continue;
                }
                for perm in present.permutations() {
                    'row: for row in 0..region.grid.nrows {
                        for col in 0..region.grid.ncols {
                            let coord = Coord {
                                row: row as i32,
                                col: col as i32,
                            };
                            if region.grid.can_place(&perm, &coord) {
                                // perm.print();
                                // println!("can place at {:?}", coord);
                                let mut present_counts = region.present_counts.clone();
                                present_counts[i] -= 1;
                                // println!("{:?}", present_counts);

                                let mut grid = region.grid.clone();
                                grid.place(&perm, &coord);
                                // grid.print();
                                if present_counts.iter().sum::<usize>() == 0 {
                                    println!("FOCK YEAH");
                                    grid.print();
                                    return true;
                                }
                                let rowmax =
                                    cmp::max(region.rowmax, coord.row as usize + perm.nrows);
                                let colmax =
                                    cmp::max(region.colmax, coord.col as usize + perm.ncols);
                                heap.push(Region {
                                    grid,
                                    present_counts,
                                    rowmax,
                                    colmax,
                                });
                                continue 'row;
                            }
                        }
                    }
                }
            }
        }

        false
    }
}
