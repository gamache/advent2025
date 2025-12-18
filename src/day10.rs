use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use rayon::prelude::*;
use regex::Regex;
use z3::{Solver, ast::Int};

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
    let sum: i64 = machines
        .par_iter()
        .flat_map(|machine| {
            let solver = Solver::new();
            let mut presses: Vec<Int> = vec![];
            for i in 0..machine.buttons.len() {
                let fresh = Int::fresh_const(format!("button-{i}").as_ref());
                solver.assert(fresh.ge(0));
                presses.push(fresh);
            }
            for ij in 0..machine.joltages.len() {
                let joltage = machine.joltages[ij];
                let mut qty = Int::from_u64(0);
                for ib in 0..machine.buttons.len() {
                    let button = &machine.buttons[ib];
                    if button.contains(&ij) {
                        qty += &presses[ib];
                        solver.assert(presses[ib].le(joltage));
                    }
                }
                solver.assert(qty.eq(joltage));
            }

            let mut lowest: Option<i64> = None;
            let solutions = solver.solutions(presses, false);
            for solution in solutions {
                let value_sum = solution.iter().flat_map(Int::as_i64).sum();
                match lowest {
                    Some(x) if x < value_sum => (),
                    _ => {
                        lowest = Some(value_sum);
                    }
                }
            }
            lowest
        })
        .sum();

    println!("day 10 part 2: {}", sum);
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
        let mut lights = vec![];
        for _ in 0..self.lights.len() {
            lights.push(false);
        }
        heap.push(SearchState1 {
            presses: 0,
            lights: lights,
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
