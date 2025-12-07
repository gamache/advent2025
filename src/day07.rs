use crate::Coord;
use crate::Grid;

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
    };
    manifold.run();
    println!("day 07 part 1: {}", manifold.splits);
}

struct Manifold {
    grid: Grid,
    beams: Vec<Coord>,
    splits: usize,
}

impl Manifold {
    pub fn tick(&mut self) {
        let mut new_beams: Vec<Coord> = vec![];
        for beam in &self.beams {
            if !self.grid.in_bounds(beam) {
                continue;
            }

            let down = beam.down();
            if self.grid.coords.get(&down) == Some(&String::from("^")) {
                self.splits += 1;
                new_beams.push(down.left());
                new_beams.push(down.right());
            } else {
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
