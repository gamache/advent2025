use crate::Coord;

pub fn run(lines: &Vec<String>) {
    let coords: Vec<Coord> = lines
        .iter()
        .flat_map(|line| {
            let numbers: Vec<i32> = line.split(",").flat_map(|n| n.parse::<i32>()).collect();
            match numbers.len() {
                0 => None,
                _ => Some(Coord {
                    row: numbers[0],
                    col: numbers[1],
                }),
            }
        })
        .collect();
    part1(&coords);
}

fn part1(coords: &Vec<Coord>) {
    let mut largest_area = 0i64;
    'outer: for c1 in coords {
        for c2 in coords {
            if c1 == c2 {
                continue 'outer;
            }
            let area = area(c1, c2);
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    println!("day 09 part 1: {}", largest_area);
}

fn area(c1: &Coord, c2: &Coord) -> i64 {
    (1 + (c1.row - c2.row).abs() as i64) * (1 + (c1.col - c2.col).abs() as i64)
}
