use std::collections::HashSet;

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
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coord3 {
    x: i64,
    y: i64,
    z: i64,
}
impl Coord3 {
    pub fn distance(c1: &Coord3, c2: &Coord3) -> f64 {
        let square =
            (c1.x - c2.x).pow(2) + (c1.y - c2.y).pow(2) + (c1.z - c2.z).pow(2);
        (square as f64).sqrt()
    }
}

#[derive(Debug, Clone)]
struct Boxes {
    coords: Vec<Coord3>,
    circuits: HashMap<Coord3, usize>,
    distances: Vec<(f64, Coord3, Coord3)>,
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
        distances.sort_by(|(ad,_,_),(bd,_,_)| ad.total_cmp(bd));
        Self {
            coords: coords,
            circuits: HashMap::new(),
            distances: distances
        }
    }

    pub fn tick(&mut self) {
        
    }

    fn connected(&self, c1: &Coord3, c2: &Coord3) -> bool {
        self.circuit_index_for(c1) == self.circuit_index_for(c2)
    }

    fn circuit_index_for(&self, c: &Coord3) -> Option<usize> {
        let mut i = 0usize;
        for circuit in &self.circuits {
            if circuit.contains(c) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    fn connect(&mut self, c1: &Coord3, c2: &Coord3) {
        let i1 = self.circuit_index_for(c1);
        let i2 = self.circuit_index_for(c2);
        let mut circuits = self.circuits.clone();

        match (i1, i2) {
            (Some(i), Some(j)) if i == j => {
                // already connected, nothing to do
            }
            (Some(i), Some(j)) => {
                let mut low = i;
                let mut high = j;
                if j < i {
                    low = j;
                    high = i;
                }
                let lowcircuit = &mut circuits[low];
                let highcircuit = &circuits[high];
                for tuple in highcircuit.iter() {
                    lowcircuit.insert(tuple.clone());
                }
                circuits.remove(high);
            }
            (Some(i), None) | (None, Some(i)) => {
                let circuit = &mut circuits[i];
                circuit.insert(c1.clone());
                circuit.insert(c2.clone());
            }
            (None, None) => {
                let mut circuit: HashSet<Coord3> = HashSet::new();
                circuit.insert(c1.clone());
                circuit.insert(c2.clone());
                circuits.push(circuit);
            }
        }
        self.circuits = circuits;
    }
}

fn part1(coords: &Vec<Coord3>) {
    let mut boxes = Boxes::new(coords.clone());
    println!("{:?}", boxes.distances);
    println!("{}", boxes.distances.len());
}
