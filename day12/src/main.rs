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
