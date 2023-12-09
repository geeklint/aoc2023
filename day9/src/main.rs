fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let nums = line
            .split_whitespace()
            .map(|d| d.parse().unwrap())
            .collect::<Vec<i64>>();
        let mut tails = Vec::new();
        let mut diffs = nums.clone();
        while !diffs.iter().all(|&n| n == 0) {
            tails.push(*diffs.last().unwrap());
            diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        }
        sum += tails.into_iter().sum::<i64>();
    }
    dbg!(sum);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let nums = line
            .split_whitespace()
            .map(|d| d.parse().unwrap())
            .collect::<Vec<i64>>();
        let mut heads = Vec::new();
        let mut diffs = nums.clone();
        while !diffs.iter().all(|&n| n == 0) {
            heads.push(*diffs.first().unwrap());
            diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        }
        sum += heads.into_iter().rev().fold(0, |acc, head| head - acc);
    }
    dbg!(sum);
}
