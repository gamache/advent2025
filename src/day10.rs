use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use rayon::prelude::*;
use regex::Regex;

use crate::Coord;

pub fn run(lines: &Vec<String>) {
    let machines = lines.iter().map(Machine::new).collect();
    part1(&machines);
    part2(&machines);
}

fn part1(machines: &Vec<Machine>) {
    let part1: usize = machines.par_iter().map(|m| m.minimum_presses_1()).sum();
    println!("day 10 part 1: {}", part1);
}

fn part2(machines: &Vec<Machine>) {
    for machine in machines {
        let mut linsys = LinSys::from_machine(&machine);
        linsys.gaussian();
        linsys.print();
        println!("free values {:?}", linsys.free_values());
        println!("max_for {:?}", linsys.max_for(0));
        let mut fixed_values = HashMap::new();
        fixed_values.insert(0, 1);
        fixed_values.insert(1, 3);
        fixed_values.insert(3, 3);
        fixed_values.insert(4, 1);
        fixed_values.insert(5, 2);

        println!("test {:?}", linsys.test(&fixed_values));
        panic!("shit");
    }
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}
impl Machine {
    pub fn new(line: &String) -> Self {
        let lights_re = Regex::new(r"[.#]").unwrap();
        let button_re = Regex::new(r"\(([\d,]+)\)").unwrap();
        let joltages_re = Regex::new(r"\{([\d,]+)\}").unwrap();

        let lights: Vec<bool> = lights_re
            .find_iter(line)
            .map(|m| match m.as_str() {
                "." => false,
                "#" => true,
                _ => panic!("bad goal"),
            })
            .collect();

        let buttons: Vec<Vec<usize>> = button_re
            .captures_iter(line)
            .map(|caps| {
                caps[1]
                    .split(",")
                    .flat_map(|s| s.parse::<usize>())
                    .collect()
            })
            .collect();

        let joltages_caps = joltages_re.captures(line).unwrap();
        let joltages: Vec<u64> = joltages_caps[1]
            .split(",")
            .flat_map(|s| s.parse::<u64>())
            .collect();

        Self {
            lights: lights,
            buttons: buttons,
            joltages: joltages,
        }
    }

    pub fn apply_button_to_lights(lights: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
        let mut output = lights.clone();
        for i in button {
            output[*i] = !output[*i];
        }
        output
    }

    pub fn minimum_presses_1(&self) -> usize {
        let mut prev_lights: HashSet<Vec<bool>> = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push(SearchState1 {
            presses: 0,
            lights: self.lights.clone(),
        });

        while let Some(ss) = heap.pop() {
            if ss.lights == self.lights {
                return ss.presses as usize;
            }
            if prev_lights.contains(&ss.lights) {
                continue;
            }
            for button in &self.buttons {
                heap.push(SearchState1 {
                    presses: ss.presses + 1,
                    lights: Self::apply_button_to_lights(&ss.lights, button),
                })
            }
            prev_lights.insert(ss.lights);
        }
        0
    }
}

#[derive(Debug, Eq, PartialEq)]
struct SearchState1 {
    presses: u64,
    lights: Vec<bool>,
}
impl Ord for SearchState1 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.presses.cmp(&self.presses)
    }
}
impl PartialOrd for SearchState1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// A linear system of equations where every starting coefficient is 0 or 1.
struct LinSys {
    rows: Vec<Vec<i64>>,
    fixed_values: HashMap<usize, i64>,
    nvars: usize,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}
impl LinSys {
    pub fn from_machine(machine: &Machine) -> Self {
        let mut coords: HashMap<Coord, i64> = HashMap::new();
        let ncols = machine.buttons.len() + 1;
        let nrows = machine.joltages.len();

        for col in 0..machine.buttons.len() {
            for row in &machine.buttons[col] {
                coords.insert(
                    Coord {
                        row: (*row) as i32,
                        col: col as i32,
                    },
                    1,
                );
            }
        }
        for row in 0..nrows {
            coords.insert(
                Coord {
                    row: row as i32,
                    col: ncols as i32 - 1,
                },
                machine.joltages[row] as i64,
            );
        }

        let mut rows = vec![];
        for row in 0..machine.joltages.len() {
            let mut vs: Vec<i64> = vec![];
            for col in 0..(machine.buttons.len() + 1) {
                let coord = Coord {
                    row: row as i32,
                    col: col as i32,
                };
                let v = coords.get(&coord).unwrap_or(&0);
                vs.push(*v);
            }
            rows.push(vs);
        }

        Self {
            rows: rows,
            fixed_values: HashMap::new(),
            nvars: machine.buttons.len(),
            buttons: machine.buttons.clone(),
            joltages: machine.joltages.clone(),
        }
    }

    pub fn print(&self) {
        for row in &self.rows {
            println!("{:?}", row);
        }
        println!("fixed: {:?}\n", self.fixed_values);
    }

    // Does Gaussian elimination on the linear system. Answers end up
    // in `self.fixed_values`. Sometimes you end up with all the answers.
    // Sometimes you don't.
    pub fn gaussian(&mut self) {
        let mut i = 0usize;
        while i < (self.rows.len() - 1) {
            self.sort_rows();
            // println!("after sort_rows");
            // self.print();

            self.subtract(i);
            // println!("after subtract");
            // self.print();

            self.make_leading_coefficient_1(i + 1);
            // println!("after coeff1");
            // self.print();

            self.handle_fixed_values();
            // println!("after handle_fixed_values");
            // self.print();

            i += 1;
        }
    }

    fn test(&self, fixed_values: &HashMap<usize, i64>) -> Ordering {
        let mut joltages: Vec<i64> = vec![];
        for _ in 0..self.rows.len() {
            joltages.push(0);
        }
        for (i, fv) in fixed_values.into_iter() {
            for j in &self.buttons[*i] {
                joltages[*j] += *fv;
            }
        }
        println!("computed joltages {:?}", joltages);

        println!("self.joltages {:?}", self.joltages);
        let mut return_value = Ordering::Equal;
        for i in 0..joltages.len() {
            if self.joltages[i] < joltages[i] as u64 {
                return Ordering::Greater;
            }
            if self.joltages[i] > joltages[i] as u64 {
                return_value = Ordering::Less;
            }
        }
        return_value
    }

    // returns index of first nonzero value
    fn first_nonzero_index(row: &Vec<i64>) -> Option<usize> {
        for i in 0..row.len() {
            if row[i] != 0 {
                return Some(i);
            }
        }
        None
    }

    fn sort_rows(&mut self) {
        let rowlen = self.rows[0].len();
        self.rows.sort_by(|a, b| {
            Self::first_nonzero_index(a)
                .unwrap_or(rowlen)
                .cmp(&Self::first_nonzero_index(b).unwrap_or(rowlen))
        });
    }

    fn make_leading_coefficient_1(&mut self, row_index: usize) {
        let fni = match Self::first_nonzero_index(&self.rows[row_index]) {
            None => return,
            Some(v) => v,
        };
        let fnv = self.rows[row_index][fni];
        for i in fni..self.rows[row_index].len() {
            self.rows[row_index][i] /= fnv;
        }
    }

    // Subtracts the row at `index` from every row under it, so as to
    // eliminate that variable.
    fn subtract(&mut self, index: usize) {
        let fni = match Self::first_nonzero_index(&self.rows[index]) {
            None => return,
            Some(v) => v,
        };
        for i in 0..self.rows.len() {
            if i == index {
                continue;
            }
            // println!("fni={} i={} index={}", fni, i, index);
            if self.rows[i][fni] != 0 {
                let mut new_row: Vec<i64> = self.rows[i].clone();
                for ii in 0..self.rows[i].len() {
                    new_row[ii] += (0 - self.rows[i][fni]) * self.rows[index][ii];
                }
                self.rows[i] = new_row;
            }
        }
    }

    fn handle_fixed_values(&mut self) {
        loop {
            let fvs = self.fixed_values.clone();
            self.find_fixed_values();
            self.apply_fixed_values();
            if fvs == self.fixed_values {
                break;
            }
        }
    }

    fn find_fixed_values(&mut self) {
        for row in &self.rows {
            let mut nonzero_indexes: Vec<usize> = vec![];
            for i in 0..(row.len() - 1) {
                if row[i] != 0 {
                    nonzero_indexes.push(i);
                }
            }
            if nonzero_indexes.len() == 1 {
                self.fixed_values
                    .insert(nonzero_indexes.pop().unwrap(), row[row.len() - 1]);
            }
        }
    }

    fn apply_fixed_values(&mut self) {
        for i in 0..self.rows.len() {
            for (idx, val) in &self.fixed_values {
                let len = &self.rows[i].len();
                self.rows[i][len - 1] -= self.rows[i][*idx] * *val;
                self.rows[i][*idx] = 0;
            }
        }
    }

    fn free_values(&self) -> Vec<usize> {
        let mut out = vec![];
        for i in 0..self.nvars {
            if None == self.fixed_values.get(&i) {
                out.push(i);
            }
        }
        out
    }

    fn max_for(&self, index: usize) -> usize {
        self.rows
            .iter()
            .filter(|row| row[index] != 0)
            .map(|row| row.last().unwrap() * row[index])
            .min()
            .unwrap() as usize
    }
}
