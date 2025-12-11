use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

/*
use ndarray::prelude::*;
use ndarray_linalg::Solve;
 */
use rayon::prelude::*;
use regex::Regex;

pub fn run(lines: &Vec<String>) {
    let machines: Vec<Machine> = lines.iter().map(Machine::new).collect();
    let part1: usize = machines.par_iter().map(|m| m.minimum_presses_1()).sum();
    println!("day 10 part 1: {}", part1);
    // let part2: usize = machines.par_iter().map(|m| m.minimum_presses_2()).sum();
    // println!("day 10 part 2: {}", part2);

    /*
    let a: Array2<f64> = array![
        [1., 1., 1., 0.],
        [1., 0., 1., 1.],
        [1., 0., 1., 1.],
        [1., 1., 0., 0.],
        [1., 1., 1., 0.],
        [0., 0., 1., 0.],
    ];
    let b: Array1<f64> = array![10., 11., 11., 5., 10., 5.];
    let x = a.solve_into(b);
    println!("{:?}", x);
     */
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

    pub fn minimum_presses_2(&self) -> usize {
        let mut prev_joltages: HashSet<Vec<u64>> = HashSet::new();
        let mut heap = BinaryHeap::new();

        let mut empty_joltages: Vec<u64> = vec![];
        for _ in 0..self.joltages.len() {
            empty_joltages.push(0);
        }

        heap.push(SearchState2 {
            presses: 0,
            joltages: empty_joltages,
        });

        while let Some(ss) = heap.pop() {
            for button in &self.buttons {
                let new_joltages = Self::apply_button_to_joltages(&ss.joltages, button);

                if prev_joltages.contains(&new_joltages) {
                    continue;
                }
                prev_joltages.insert(new_joltages.clone());

                for i in 0..self.joltages.len() {
                    if new_joltages[i] > self.joltages[i] {
                        continue;
                    }
                }

                if new_joltages == self.joltages {
                    println!("{}", 1 + ss.presses);
                    return 1 + ss.presses as usize;
                }

                heap.push(SearchState2 {
                    presses: ss.presses + 1,
                    joltages: new_joltages,
                })
            }
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

#[derive(Debug, Eq, PartialEq)]
struct SearchState2 {
    presses: u64,
    joltages: Vec<u64>,
}
impl Ord for SearchState2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.presses.cmp(&self.presses)
    }
}
impl PartialOrd for SearchState2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
