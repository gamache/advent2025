use crate::Coord;
use crate::Grid;
use std::collections::HashMap;

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);
    part1(&grid);
}

fn part1(grid: &Grid) {
    let start = grid.find("S").unwrap();
    let mut manifold = Manifold {
        grid: grid.clone(),
        beams: vec![start],
        splits: 0,
        path_counts: HashMap::new(),
    };
    manifold.run();
    println!("day 07 part 1: {}", manifold.splits);

    let part2: usize = manifold
        .path_counts
        .iter()
        .filter(|(c, _)| c.row as usize == manifold.grid.nrows - 1)
        .map(|(_, paths)| paths)
        .sum();
    println!("day 07 part 2: {}", part2);
}

struct Manifold {
    grid: Grid,
    beams: Vec<Coord>,
    splits: usize,
    path_counts: HashMap<Coord, usize>,
}

impl Manifold {
    pub fn tick(&mut self) {
        let mut new_beams: Vec<Coord> = vec![];
        for beam in &self.beams {
            if !self.grid.in_bounds(beam) {
                continue;
            }

            let paths_to_here = *self.path_counts.get(beam).unwrap_or(&1);
            let down = beam.down();
            if self.grid.coords.get(&down) == Some(&String::from("^")) {
                self.splits += 1;

                let left_paths = *self.path_counts.get(&down.left()).unwrap_or(&0);
                self.path_counts
                    .insert(down.left(), left_paths + paths_to_here);
                new_beams.push(down.left());

                let right_paths = *self.path_counts.get(&down.right()).unwrap_or(&0);
                self.path_counts
                    .insert(down.right(), right_paths + paths_to_here);
                new_beams.push(down.right());
            } else {
                let down_paths = *self.path_counts.get(&down).unwrap_or(&0);
                self.path_counts.insert(down, down_paths + paths_to_here);
                new_beams.push(down);
            }
        }
        new_beams.sort();
        new_beams.dedup();
        self.beams = new_beams;
    }

    pub fn run(&mut self) {
        loop {
            if self.beams.len() == 0 {
                return;
            }
            self.tick();
        }
    }
}
