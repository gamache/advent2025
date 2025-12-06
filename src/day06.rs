use regex::Regex;

pub fn run(lines: &Vec<String>) {
    let number_re = Regex::new(r"\d+").unwrap();
    let operator_re = Regex::new(r"[-+*/]").unwrap();

    let mut number_lines = lines.clone();
    let operator_line = number_lines.get(number_lines.len() - 1).unwrap().clone();

    number_lines.remove(number_lines.len() - 1);

    let numbers: Vec<Vec<i64>> = number_lines
        .iter()
        .map(|line| {
            number_re
                .find_iter(line)
                .map(|n| n.as_str().parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let operators: Vec<String> = operator_re
        .find_iter(&operator_line)
        .map(|o| o.as_str().into())
        .collect();

    let part1: i64 = operators
        .iter()
        .enumerate()
        .map(|(col, op)| {
            let ns = numbers.iter().map(|nv| nv.get(col).unwrap().clone());

            match op.as_str() {
                "+" => ns.reduce(|acc, n| acc + n).unwrap(),
                "-" => ns.reduce(|acc, n| acc - n).unwrap(),
                "*" => ns.reduce(|acc, n| acc * n).unwrap(),
                "/" => ns.reduce(|acc, n| acc / n).unwrap(),
                _ => panic!("bad op"),
            }
        })
        .sum();
    println!("day 06 part 1: {}", part1);
}
