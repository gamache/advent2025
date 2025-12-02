use regex::Regex;

pub fn run(input: &String) {
    let range_re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut sum1 = 0u64;
    let mut sum2 = 0u64;

    for caps in range_re.captures_iter(input.as_str()) {
        let start = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let end = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        for x in start..(end + 1) {
            if invalid1(x) {
                sum1 += x;
            }
            if invalid2(x) {
                sum2 += x;
            }
        }
    }
    println!("day 02 part 1: {}", sum1);
    println!("day 02 part 2: {}", sum2);
}

fn invalid1(x: u64) -> bool {
    let xstr = x.to_string();
    let len = xstr.len();
    if len % 2 == 0 && xstr[0..(len / 2)] == xstr[(len / 2)..len] {
        return true;
    }
    false
}

fn invalid2(x: u64) -> bool {
    let xstr = x.to_string();
    let len = xstr.len();

    for n in 1..(len / 2 + 1) {
        if len % n == 0 {
            let ntimes = len / n;
            if xstr[0..n].repeat(ntimes) == xstr {
                return true;
            }
        }
    }

    false
}
