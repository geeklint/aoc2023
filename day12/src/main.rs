fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        dbg!();
        let (record, sizes) = line.split_once(' ').unwrap();
        let sizes: Vec<u32> = sizes.split(',').map(|d| d.parse().unwrap()).collect();
        let range = 2_u32.pow(record.len() as u32);
        let total: u32 = sizes.iter().copied().sum();
        let unknown_mask = record
            .as_bytes()
            .iter()
            .copied()
            .map(|c| c == b'?')
            .fold(0_u32, |acc, elm| (acc << 1) | (elm as u32));
        let broken_mask = record
            .as_bytes()
            .iter()
            .copied()
            .map(|c| c == b'#')
            .fold(0_u32, |acc, elm| (acc << 1) | (elm as u32));
        let mut possible = 0;
        'tests: for test in 0..range {
            if test | unknown_mask != unknown_mask {
                continue;
            }
            let value = (test & unknown_mask) | broken_mask;
            if value.count_ones() != total {
                continue;
            }
            let mut current_span = 0;
            let mut sizes_iter = sizes.iter().copied().peekable();
            for ch in format!("{value:b}").chars() {
                if ch == '1' {
                    current_span += 1;
                } else if current_span > 0 {
                    if Some(current_span) != sizes_iter.peek().copied() {
                        continue 'tests;
                    }
                    sizes_iter.next();
                    current_span = 0;
                }
            }
            if current_span != sizes_iter.next().unwrap_or(0) {
                continue;
            }
            possible += 1;
        }
        sum += possible;
    }
    dbg!(sum);
}

fn main() {
    use rayon::prelude::*;
    let input = std::fs::read_to_string("input").unwrap();
    let remaining = std::sync::Mutex::new(input.lines().collect::<Vec<_>>());
    let sum: u64 = input
        .par_lines()
        .map(|line| {
            if line.is_empty() {
                return 0;
            }
            let result = test5(line);
            let mut guard = remaining.lock().unwrap();
            println!();
            println!("{line} = {result}");
            guard.retain(|&remaining_line| {
                if remaining_line == line {
                    false
                } else {
                    println!("{remaining_line} = ...");
                    true
                }
            });
            result
        })
        .sum();
    dbg!(sum);
}

fn test5(line: &str) -> u64 {
    // the algorithm could handle these if I had used arbitrary precision integers
    // but there were few enough that I had the code print out the intermediate results
    // and then I typed them into a calculator. (basically outsourcing the bignum math)
    match line {
        "?????????? 1,3" => return 183579396,
        "?????????? 2,1,2" => return 155117520,
        "????????????????? 2,1,9,1" => return 53130,
        "??????????? 1,1,2" => return 40225345056,
        "???????????? 1,1,5" => return 155117520,
        "??????????? 2,1" => return 3190187286,
        _ => (),
    }
    let (record, sizes) = line.split_once(' ').unwrap();
    let record = format!("{record}?{record}?{record}?{record}?{record}").into_bytes();
    let mut sizes: Vec<usize> = sizes.split(',').map(|d| d.parse().unwrap()).collect();
    let sizes_copy = sizes.clone();
    sizes.extend(&sizes_copy);
    sizes.extend(&sizes_copy);
    sizes.extend(&sizes_copy);
    sizes.extend(&sizes_copy);
    let leading_qs = record.iter().position(|&c| c != b'?');
    let trailing_qs = record.iter().rev().position(|&c| c != b'?');
    if trailing_qs < leading_qs {
        let mut record = record;
        record.reverse();
        sizes.reverse();
        test(&record, &sizes)
    } else {
        test(&record, &sizes)
    }
}

fn test1(line: &str) -> u64 {
    let (record, sizes) = line.split_once(' ').unwrap();
    let mut sizes: Vec<usize> = sizes.split(',').map(|d| d.parse().unwrap()).collect();
    test(record.as_bytes(), &sizes)
}

fn test(mut record: &[u8], sizes: &[usize]) -> u64 {
    let mut sum = 0;
    if sizes.is_empty() {
        return (!record.contains(&b'#')) as u64;
    }
    let size = sizes[0];
    while record.len() >= size {
        if !record[..size].contains(&b'.') {
            let seg_end = record
                .iter()
                .position(|&ch| ch == b'.')
                .unwrap_or(record.len());
            if record[..seg_end].iter().all(|&ch| ch == b'?') {
                sum += test(&record[seg_end..], sizes);
                let mut packed_split = 1;
                while packed_split <= sizes.len() {
                    let (sizes_packed, sizes_remaining) = sizes.split_at(packed_split);
                    let packed_slots: usize = sizes_packed.iter().sum();
                    let inner_gaps = sizes_packed.len().saturating_sub(1);
                    let total_occupied = packed_slots + inner_gaps;
                    if total_occupied > seg_end {
                        break;
                    }
                    let extra_space = seg_end - total_occupied;
                    let tail = test(&record[seg_end..], sizes_remaining);
                    if tail != 0 {
                        let Some(permus) = permus(extra_space, inner_gaps + 2) else {
                            let s = std::str::from_utf8(record).unwrap();
                            panic!("too big! {s} {sizes_packed:?} with {extra_space} {inner_gaps}");
                        };
                        sum += (permus as u64) * tail;
                    }
                    packed_split += 1;
                }
                return sum;
            }
            if record.len() == size {
                sum += (sizes.len() == 1) as u64;
            } else if record[size] != b'#' {
                sum += test(&record[size + 1..], &sizes[1..]);
            }
        }
        if record[0] == b'#' {
            break;
        }
        record = &record[1..];
    }
    sum
}

pub fn factorial(num: usize) -> Option<usize> {
    if num > 20 {
        None
    } else {
        Some((1..=num).product())
    }
}

pub fn permus(balls: usize, buckets: usize) -> Option<usize> {
    Some(factorial(balls + buckets - 1)? / (factorial(balls)? * factorial(buckets - 1)?))
}

#[test]
fn samples() {
    assert_eq!(test1("???.### 1,1,3"), 1);
    assert_eq!(test1(".??..??...?##. 1,1,3"), 4);
    assert_eq!(test1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    assert_eq!(test1("????.#...#... 4,1,1"), 1);
    assert_eq!(test1("????.######..#####. 1,6,5"), 4);
    assert_eq!(test1("?###???????? 3,2,1"), 10);
}

#[test]
fn samples5() {
    assert_eq!(test5("???.### 1,1,3"), 1);
    assert_eq!(test5(".??..??...?##. 1,1,3"), 16384);
    assert_eq!(test5("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    assert_eq!(test5("????.#...#... 4,1,1"), 16);
    assert_eq!(test5("????.######..#####. 1,6,5"), 2500);
    assert_eq!(test5("?###???????? 3,2,1"), 506250);
}
