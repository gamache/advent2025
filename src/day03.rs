pub fn run(lines: &Vec<String>) {
    let mut sum1 = 0u64;
    let mut sum2 = 0u64;
    for line in lines {
        sum1 += jolts1(line);
        sum2 += jolts2(line);
    }
    println!("day 03 part 1: {}", sum1);
    println!("day 03 part 2: {}", sum2);
}

fn jolts1(line: &String) -> u64 {
    let chars: Vec<char> = line.chars().collect();
    let first = highest_digit(&chars, 0, chars.len() - 1);
    let second = highest_digit(&chars, first.1 + 1, chars.len());

    format!("{}{}", first.0, second.0).parse::<u64>().unwrap()
}

fn highest_digit(chars: &Vec<char>, start: usize, end: usize) -> (char, usize) {
    let mut index = start;
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

fn jolts2(line: &String) -> u64 {
    let chars: Vec<char> = line.chars().collect();
    let mut numstr = String::from("");
    let mut index = 0usize;

    for ci in 0..12 {
        let (c, i) = highest_digit(&chars, index, chars.len() - 11 + ci);
        numstr = format!("{}{}", numstr, c);
        index = i + 1;
    }

    numstr.parse::<u64>().unwrap()
}
