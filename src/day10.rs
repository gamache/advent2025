use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use regex::Regex;

pub fn run(lines: &Vec<String>) {
    let machines: Vec<Machine> = lines.iter().map(Machine::new).collect();
    println!("{:?}", machines[0]);
    let presses: usize = machines.iter().map(|m| m.minimum_presses()).sum();
    println!("day 10 part 1: {}", presses);
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    goal: Vec<bool>,
    buttons: Vec<Vec<bool>>,
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

        let buttons: Vec<Vec<bool>> = button_re
            .captures_iter(line)
            .map(|caps| {
                let indexes: Vec<usize> = caps[1]
                    .split(",")
                    .flat_map(|s| s.parse::<usize>())
                    .collect();
                let mut button = lights.clone();
                for i in indexes {
                    button[i] = true;
                }
                button
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

    pub fn apply_button(lights: &Vec<bool>, button: &Vec<bool>) -> Vec<bool> {
        let mut output = lights.clone();
        for i in 0..lights.len() {
            output[i] ^= button[i];
        }
        output
    }

    pub fn minimum_presses(&self) -> usize {
        let mut prev_lights: HashSet<Vec<bool>> = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push(SearchState {
            presses: 0,
            lights: self.lights.clone(),
        });

        while let Some(ss) = heap.pop() {
            if ss.lights == self.goal {
                return ss.presses;
            }
            if prev_lights.contains(&ss.lights) {
                continue;
            }
            for button in &self.buttons {
                heap.push(SearchState {
                    presses: ss.presses + 1,
                    lights: Self::apply_button(&ss.lights, button),
                })
            }
            prev_lights.insert(ss.lights);
        }
        0
    }
}

#[derive(Eq, PartialEq)]
struct SearchState {
    presses: usize,
    lights: Vec<bool>,
}
impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.presses.cmp(&self.presses)
    }
}
impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
