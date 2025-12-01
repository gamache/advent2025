use regex::Regex;

pub fn run(lines: &Vec<String>) {
    let line_re = Regex::new(r"([RL])(\d+)").unwrap();
    let mut n = 50i32;
    let mut zero_endings = 0u32;
    let mut zero_landings = 0u32;

    for line in lines {
        let caps = line_re.captures(line).unwrap();
        let direction = caps.get(1).unwrap().as_str();
        let distance = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

        let increment = match direction {
            "R" => 1,
            "L" => -1,
            _ => panic!("bad direction"),
        };

        for _ in 0..distance {
            n += increment;
            n %= 100;
            if n == 0 {
                zero_landings += 1;
            }
        }
        if n == 0 {
            zero_endings += 1;
        }
    }

    println!("day 01 part 1: {}", zero_endings);
    println!("day 01 part 2: {}", zero_landings);
}
