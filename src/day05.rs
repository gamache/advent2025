#[derive(Debug, Clone)]
struct Range {
    low: u64,
    high: u64,
}
impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut splitter = s.splitn(2, "-");
        let low = splitter.next().unwrap().parse::<u64>().unwrap();
        let high = splitter.next().unwrap().parse::<u64>().unwrap();
        Range { low, high }
    }
}
impl Range {
    pub fn contains(&self, n: u64) -> bool {
        n >= self.low && n <= self.high
    }
}

pub fn run(input: &String) {
    let mut splitter = input.splitn(2, "\n\n");
    let ranges: Vec<Range> = splitter.next().unwrap().lines().map(|l| l.into()).collect();
    let ingredients: Vec<u64> = splitter
        .next()
        .unwrap()
        .lines()
        .flat_map(|l| l.parse::<u64>())
        .collect();
    part1(&ranges, &ingredients);
    part2(&ranges);
}

fn part1(ranges: &Vec<Range>, ingredients: &Vec<u64>) {
    let mut fresh = 0usize;
    'ing_loop: for ingredient in ingredients {
        for range in ranges {
            if range.contains(*ingredient) {
                fresh += 1;
                continue 'ing_loop;
            }
        }
    }
    println!("day 05 part 1: {}", fresh);
}

fn part2(ranges: &Vec<Range>) {
    let mut rs = ranges.clone();
    rs.sort_by(|a, b| a.low.cmp(&b.low));

    let mut count = 0u64;
    let mut cur_low = 0u64;
    let mut cur_high = 0u64;

    for range in rs {
        if range.low > cur_high {
            if cur_high > 0 {
                count += cur_high - cur_low + 1;
            }
            cur_low = range.low;
            cur_high = range.high;
        } else {
            if range.high > cur_high {
                cur_high = range.high;
            }
        }
    }
    count += cur_high - cur_low + 1;
    println!("day 05 part 2: {}", count);
}
