fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut lines = input.lines();
    let times_str = lines.next().unwrap().strip_prefix("Time:").unwrap();
    let distances_str = lines.next().unwrap().strip_prefix("Distance:").unwrap();
    let times = times_str
        .split_whitespace()
        .map(|d| d.parse::<u32>().unwrap());
    let distances = distances_str
        .split_whitespace()
        .map(|d| d.parse::<u32>().unwrap());
    let mut result = 1;
    for (time, distance) in times.zip(distances) {
        let mut valid = 0;
        for test in 1..time {
            let score = (time - test) * test;
            if score > distance {
                valid += 1;
            }
        }
        result *= dbg!(valid);
    }
    dbg!(result);
}

fn main() {
    let mut input = std::fs::read_to_string("input").unwrap();
    input.retain(|c| c != ' ');
    let mut lines = input.lines();
    let times_str = lines.next().unwrap().strip_prefix("Time:").unwrap();
    let distances_str = lines.next().unwrap().strip_prefix("Distance:").unwrap();
    let times = times_str
        .split_whitespace()
        .map(|d| d.parse::<u64>().unwrap());
    let distances = distances_str
        .split_whitespace()
        .map(|d| d.parse::<u64>().unwrap());
    let mut result = 1;
    for (time, distance) in times.zip(distances) {
        let mut valid = 0;
        for test in 1..time {
            let score = (time - test) * test;
            if score > distance {
                valid += 1;
            }
        }
        result *= dbg!(valid);
    }
    dbg!(result);
}
