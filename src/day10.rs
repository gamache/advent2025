use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use rayon::prelude::*;
use regex::Regex;

pub fn run(lines: &Vec<String>) {
    let machines: Vec<Machine> = lines.iter().map(Machine::new).collect();
    let part1: usize = machines.par_iter().map(|m| m.minimum_presses_1()).sum();
    println!("day 10 part 1: {}", part1);
    let part2: usize = machines.par_iter().map(|m| m.minimum_presses_2()).sum();
    println!("day 10 part 2: {}", part2);
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}
impl Machine {
    pub fn new(line: &String) -> Self {
        let goal_re = Regex::new(r"[.#]").unwrap();
        let button_re = Regex::new(r"\(([\d,]+)\)").unwrap();
        let joltages_re = Regex::new(r"\{([\d,]+)\}").unwrap();

        let mut lights: Vec<bool> = vec![];
        let goal: Vec<bool> = goal_re
            .find_iter(line)
            .map(|m| {
                lights.push(false);
                match m.as_str() {
                    "." => false,
                    "#" => true,
                    _ => panic!("bad goal"),
                }
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
            goal: goal,
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

    pub fn apply_button_to_joltages(joltages: &Vec<u64>, button: &Vec<usize>) -> Vec<u64> {
        let mut output = joltages.clone();
        for i in button {
            output[*i] += 1;
        }
        output
    }

    pub fn minimum_presses_1(&self) -> usize {
        let mut prev_lights: HashSet<Vec<bool>> = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push(SearchState {
            presses: 0,
            remaining_cost: 0,
            lights: self.lights.clone(),
            joltages: vec![],
        });

        while let Some(ss) = heap.pop() {
            if ss.lights == self.goal {
                return ss.presses as usize;
            }
            if prev_lights.contains(&ss.lights) {
                continue;
            }
            for button in &self.buttons {
                heap.push(SearchState {
                    presses: ss.presses + 1,
                    remaining_cost: 0,
                    lights: Self::apply_button_to_lights(&ss.lights, button),
                    joltages: vec![],
                })
            }
            prev_lights.insert(ss.lights);
        }
        0
    }

    pub fn minimum_presses_2(&self) -> usize {
        let mut prev_joltages: HashSet<Vec<u64>> = HashSet::new();
        let mut heap = BinaryHeap::new();

        let mut empty_joltages: Vec<u64> = vec![];
        for _ in 0..self.joltages.len() {
            empty_joltages.push(0);
        }

        heap.push(SearchState {
            presses: 0,
            remaining_cost: 0,
            lights: vec![],
            joltages: empty_joltages,
        });

        while let Some(ss) = heap.pop() {
            if ss.joltages == self.joltages {
                println!("{}", ss.presses);
                return ss.presses as usize;
            }
            for i in 0..self.joltages.len() {
                if ss.joltages[i] > self.joltages[i] {
                    continue;
                }
            }
            if prev_joltages.contains(&ss.joltages) {
                continue;
            }
            for button in &self.buttons {
                heap.push(SearchState {
                    presses: ss.presses + 1,
                    remaining_cost: 0,
                    lights: vec![],
                    joltages: Self::apply_button_to_joltages(&ss.joltages, button),
                })
            }
            prev_joltages.insert(ss.joltages);
        }
        0
    }
}

#[derive(Debug, Eq, PartialEq)]
struct SearchState {
    presses: u64,
    remaining_cost: u64,
    lights: Vec<bool>,
    joltages: Vec<u64>,
}
impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.presses + other.remaining_cost).cmp(&(self.presses + self.remaining_cost))
    }
}
impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
