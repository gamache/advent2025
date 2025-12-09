use std::collections::HashSet;
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
    part2(&coords);
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

fn part2(reds: &Vec<Coord>) {
    let greens = greens(reds);
    println!("{:?}", greens);
    println!("greens.len() = {}", greens.len());
   
}

fn greens(reds: &Vec<Coord>) -> HashSet<Coord> {
    let mut greens: HashSet<Coord> = HashSet::new();
    for i in 0..reds.len() {
        let red1 = reds[i];
        let red2 = reds[(i + 1) % reds.len()];

        if red1.col > red2.col {
            for c in (red2.col + 1)..red1.col {
                greens.insert(Coord {
                    row: red1.row,
                    col: c,
                });
            }
        } else if red1.col < red2.col {
            for c in (red1.col + 1)..red2.col {
                greens.insert(Coord {
                    row: red1.row,
                    col: c,
                });
            }
        } else if red1.row > red2.row {
            for r in (red2.row + 1)..red1.row {
                greens.insert(Coord {
                    row: r,
                    col: red1.col,
                });
            }
        } else if red1.row > red2.row {
            for r in (red1.row + 1)..red2.row {
                greens.insert(Coord {
                    row: r,
                    col: red1.col,
                });
            }
        }
    }

    let redset: HashSet<Coord> = reds.iter().map(|c| c.clone()).collect();
    let inside_coord = find_coord_inside(&redset, &greens).unwrap();
    println!("inside_coord {:?}", inside_coord);
    flood_fill(&redset, &mut greens, &inside_coord);

    greens
}

fn find_coord_inside(redset: &HashSet<Coord>, greens: &HashSet<Coord>) -> Option<Coord> {
    let minrow = redset.iter().map(|c| c.row).min().unwrap();
    let maxrow = redset.iter().map(|c| c.row).max().unwrap();
    let mincol = redset.iter().map(|c| c.col).min().unwrap();
    let maxcol = redset.iter().map(|c| c.col).max().unwrap();
    println!("row {} {} col {} {}", minrow, maxrow, mincol, maxcol);

    for row in ((maxrow + minrow)/2)..(maxrow+1) {
        println!("find_coord_inside row={}", row);
        for col in ((maxcol + mincol)/2)..(maxcol+1) {
            let mut count = 0usize;
            for r in 0..row {
                let c = Coord { row: r, col: col };
                if redset.contains(&c) || greens.contains(&c) {
                    count += 1;
                }
            }
            if count % 2 == 1 {
                return Some(Coord { row: row, col: col });
            }
        }
    }
    None
}


fn flood_fill(redset: &HashSet<Coord>, greens: &mut HashSet<Coord>, inside_coord: &Coord) {
    let mut coords: Vec<Coord> = vec![inside_coord.clone()];

    loop {
        match coords.pop() {
            None => break,
            Some(c) => {
                if !redset.contains(&c) && !greens.contains(&c) {
                    greens.insert(c);
                    coords.push(c.up());
                    coords.push(c.down());
                    coords.push(c.left());
                    coords.push(c.right());
                }
            }
        }
    }
}
