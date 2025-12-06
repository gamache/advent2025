use regex::Regex;
use std::ops::Range;

pub fn run(input: &String) {
    let mut number_lines: Vec<String> = input.trim().lines().map(|s| s.into()).collect();
    let operator_line = number_lines.get(number_lines.len() - 1).unwrap().clone();

    number_lines.remove(number_lines.len() - 1);

    part1(&number_lines, &operator_line);
    part2(&number_lines, &operator_line);
}

fn part1(number_lines: &Vec<String>, operator_line: &String) {
    let number_re = Regex::new(r"\d+").unwrap();
    let operator_re = Regex::new(r"[-+*/]").unwrap();

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

fn part2(number_lines: &Vec<String>, operator_line: &String) {
    let mut right_col = number_lines.get(0).unwrap().len();

    let mut ops: Vec<(usize, char)> = operator_line
        .chars()
        .enumerate()
        .filter(|(_col, c)| *c != ' ')
        .collect();
    ops.sort_by(|(acol, _), (bcol, _)| bcol.cmp(acol));

    let part2: i64 = ops
        .iter()
        .map(|(col, c)| {
            let ns = numbers_at_cols(number_lines, *col..right_col).into_iter();
            right_col = *col;
            match c {
                '+' => ns.reduce(|acc, n| acc + n).unwrap(),
                '-' => ns.reduce(|acc, n| acc - n).unwrap(),
                '*' => ns.reduce(|acc, n| acc * n).unwrap(),
                '/' => ns.reduce(|acc, n| acc / n).unwrap(),
                _ => panic!("bad op"),
            }
        })
        .sum();
    println!("day 06 part 2: {}", part2);
}

fn numbers_at_cols(number_lines: &Vec<String>, cols: Range<usize>) -> Vec<i64> {
    cols.flat_map(|col| number_at_col(number_lines, col))
        .collect()
}

fn number_at_col(number_lines: &Vec<String>, col: usize) -> Option<i64> {
    let digits: Vec<String> = number_lines
        .iter()
        .map(|line| line.chars().nth(col).unwrap().to_string())
        .collect();
    let digit_str: String = digits.join("").trim().to_string();
    if digit_str.len() == 0 {
        return None;
    }
    Some(digit_str.parse::<i64>().unwrap())
}
