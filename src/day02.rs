use regex::Regex;

pub fn run(input: &String) {
    let range_re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut sum = 0u64;

    for caps in range_re.captures_iter(input.as_str()) {
        let start = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let end = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        for x in start..(end+1) {
            if invalid(x) {
                sum += x;
            }
        }
    }
    println!("day 02 part 1: {}", sum);
}


fn invalid(x: u64) -> bool {
    let xstr = x.to_string();
    let len = xstr.len();
    if len % 2 == 0 && xstr[0..(len/2)] == xstr[(len/2)..len] {
        return true;
    }
    false
}
