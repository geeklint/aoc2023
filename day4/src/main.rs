use std::collections::VecDeque;

fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    let sum: u32 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let colon = line.find(':').unwrap();
            let body = &line[colon + 1..];
            let mut winning: u128 = 0;
            let mut have: u128 = 0;
            let mut dest = &mut winning;
            for word in body.split_whitespace() {
                if word == "|" {
                    dest = &mut have;
                } else if let Ok(num) = word.parse::<u32>() {
                    *dest |= 1 << num;
                } else {
                    panic!()
                }
            }
            (1 << (winning & have).count_ones()) >> 1
        })
        .sum();
    dbg!(sum);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut future_copies = VecDeque::new();
    let sum: u32 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let copies = future_copies.pop_front().unwrap_or(0);
            let colon = line.find(':').unwrap();
            let body = &line[colon + 1..];
            let mut winning: u128 = 0;
            let mut have: u128 = 0;
            let mut dest = &mut winning;
            for word in body.split_whitespace() {
                if word == "|" {
                    dest = &mut have;
                } else if let Ok(num) = word.parse::<u32>() {
                    *dest |= 1 << num;
                } else {
                    panic!()
                }
            }
            let wins = (winning & have).count_ones() as usize;
            while future_copies.len() < wins {
                future_copies.push_back(0);
            }
            for fc in future_copies.iter_mut().take(wins) {
                *fc += 1 + copies;
            }
            dbg!(1 + copies)
        })
        .sum();
    dbg!(sum);
}
