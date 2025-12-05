#[derive(Debug)]
struct Range {
    low: u64,
    high: u64
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
    let ingredients: Vec<u64> = splitter.next().unwrap().lines().flat_map(|l| l.parse::<u64>()).collect();
    let mut fresh1 = 0usize;

    'ing_loop: for ingredient in &ingredients {
        for range in &ranges {
            if range.contains(*ingredient) {
                fresh1 += 1;
                continue 'ing_loop;
            }
        }
    }
    println!("day 05 part 1: {}", fresh1);
}
