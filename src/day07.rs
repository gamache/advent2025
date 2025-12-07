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
    let start_splitters = count_splitters(grid);
    manifold.run();
    let end_splitters = count_splitters(&manifold.grid);
    manifold.grid.print();

    // two ways of counting splits, off by one, both wrong :(
    println!("day 07 part 1: {}", manifold.splits);
    // 1528 too low
    println!("day 07 part 1: {}", start_splitters - end_splitters);
    // 1529 too low
}

fn count_splitters(grid: &Grid) -> usize {
    let mut count = 0usize;
    for coord in grid.all_coords() {
        if grid.coords.get(&coord) == Some(&String::from("^")) {
            count += 1;
        }
    }
    count
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

                self.grid.coords.insert(down.left(), String::from("|"));
                self.grid.coords.insert(down.right(), String::from("|"));
                self.grid.coords.insert(down, String::from("X"));
            } else {
                new_beams.push(down);

                self.grid.coords.insert(down, String::from("|"));
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
