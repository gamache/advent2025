use std::collections::HashMap;

pub fn run(lines: &Vec<String>) {
    let coords: Vec<Coord3> = lines
        .iter()
        .map(|line| {
            let numbers: Vec<i64> = line.split(",").flat_map(|n| n.parse::<i64>()).collect();
            Coord3 {
                x: *numbers.get(0).unwrap(),
                y: *numbers.get(1).unwrap(),
                z: *numbers.get(2).unwrap(),
            }
        })
        .collect();
    part1(&coords);
    part2(&coords);
}

fn part1(coords: &Vec<Coord3>) {
    let mut boxes = Boxes::new(coords.clone());
    for _ in 0..1000 {
        boxes.tick();
    }
    let mut lengths: Vec<usize> = boxes
        .circuit_lengths()
        .iter()
        .map(|(_circuit, length)| *length)
        .collect();
    lengths.sort_by(|a, b| b.cmp(a));
    println!("day 08 part 1: {}", lengths[0] * lengths[1] * lengths[2]);
}

fn part2(coords: &Vec<Coord3>) {
    let mut boxes = Boxes::new(coords.clone());
    loop {
        if boxes.distances.len() == 0 {
            break;
        }
        boxes.tick();
    }
    println!("day 08 part 2: {}", boxes.last_x_times_x);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coord3 {
    x: i64,
    y: i64,
    z: i64,
}
impl Coord3 {
    pub fn distance(c1: &Coord3, c2: &Coord3) -> f64 {
        let square = (c1.x - c2.x).pow(2) + (c1.y - c2.y).pow(2) + (c1.z - c2.z).pow(2);
        (square as f64).sqrt()
    }
}

#[derive(Debug, Clone)]
struct Boxes {
    circuits: HashMap<Coord3, usize>,
    circuit_index: usize,
    distances: Vec<(f64, Coord3, Coord3)>,
    last_x_times_x: i64,
    circuit_count: usize,
}
impl Boxes {
    pub fn new(coords: Vec<Coord3>) -> Self {
        let mut distances: Vec<(f64, Coord3, Coord3)> = vec![];
        'outer: for c1 in &coords {
            for c2 in &coords {
                if c1 == c2 {
                    continue 'outer;
                }
                let d = Coord3::distance(c1, c2);
                distances.push((d, c1.clone(), c2.clone()));
            }
        }

        // longest first
        distances.sort_by(|(ad, _, _), (bd, _, _)| bd.total_cmp(ad));

        Self {
            circuits: HashMap::new(),
            circuit_index: 0,
            distances: distances,
            last_x_times_x: 0,
            circuit_count: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.distances.len() == 0 {
            return;
        }
        let (_d, c1, c2) = self.distances.remove(self.distances.len() - 1);
        self.connect(&c1, &c2);
    }

    fn connect(&mut self, c1: &Coord3, c2: &Coord3) {
        let mut circuits = self.circuits.clone();
        let circuit1 = self.circuits.get(c1);
        let circuit2 = self.circuits.get(c2);

        match (circuit1, circuit2) {
            (Some(i), Some(j)) if i == j => {
                // already connected, nothing to do
            }
            (Some(i), Some(j)) => {
                for (coord, circuit) in &self.circuits {
                    if *circuit == *j {
                        circuits.insert(coord.clone(), *i);
                    }
                }
                self.circuit_count -= 1;
            }
            (Some(i), None) | (None, Some(i)) => {
                circuits.insert(c1.clone(), *i);
                circuits.insert(c2.clone(), *i);
                self.last_x_times_x = c1.x * c2.x;
            }
            (None, None) => {
                circuits.insert(c1.clone(), self.circuit_index);
                circuits.insert(c2.clone(), self.circuit_index);
                self.circuit_index += 1;
                self.circuit_count += 1;
            }
        }
        self.circuits = circuits;
    }

    pub fn circuit_lengths(&self) -> HashMap<usize, usize> {
        let mut output = HashMap::new();
        for (_coord, circuit) in &self.circuits {
            let new_count = match output.get(circuit) {
                Some(n) => n + 1,
                None => 1,
            };
            output.insert(*circuit, new_count);
        }
        output
    }
}
