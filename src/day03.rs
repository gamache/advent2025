pub fn run(lines: &Vec<String>) {
    let mut sum = 0u64;
    for line in lines {
        sum += jolts(line);
    }
    println!("day 03 part 1: {}", sum);
}

fn jolts(line: &String) -> u64 {
    let chars: Vec<char> = line.chars().collect();
    let first = highest_digit(&chars, 0, chars.len() - 1);
    let second = highest_digit(&chars, first.1 + 1, chars.len());

    format!("{}{}", first.0, second.0).parse::<u64>().unwrap()
}

fn highest_digit(chars: &Vec<char>, start: usize, end: usize) -> (char, usize) {
    let mut index = 0usize;
    let mut highest_digit = ' ';
    let mut highest_digit_index = 0usize;

    for c in &chars[start..end] {
        if *c > highest_digit {
            highest_digit = *c;
            highest_digit_index = index;
        }
        index += 1;
    }

    (highest_digit, highest_digit_index)
}
