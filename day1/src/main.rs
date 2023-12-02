fn main1() {
    let mut input = std::fs::read("input").unwrap();
    input.retain(|ch| matches!(ch, b'\n' | b'0'..=b'9'));
    let mut tens = 0_u32;
    let mut ones = 0_u32;
    for line in input.split(|&ch| ch == b'\n') {
        if line.is_empty() {
            continue;
        }
        tens += u32::from(line.first().unwrap() - b'0');
        ones += u32::from(line.last().unwrap() - b'0');
    }
    println!("{}", u128::from(tens) * 10 + u128::from(ones));
}

fn main() {
    let words = [
        &b"one"[..],
        b"two",
        b"three",
        b"four",
        b"five",
        b"six",
        b"seven",
        b"eight",
        b"nine",
        b"1",
        b"2",
        b"3",
        b"4",
        b"5",
        b"6",
        b"7",
        b"8",
        b"9",
    ];
    let mut input = std::fs::read("input").unwrap();
    let mut tens = 0_u32;
    let mut ones = 0_u32;
    for mut line in input.split(|&ch| ch == b'\n') {
        if line.is_empty() {
            continue;
        }
        'trim_start: loop {
            for word in words {
                if line.starts_with(word) {
                    break 'trim_start;
                }
            }
            line = &line[1..];
        }
        'trim_end: loop {
            for word in words {
                if line.ends_with(word) {
                    break 'trim_end;
                }
            }
            line = line.split_last().unwrap().1;
        }
        for (i, word) in words.into_iter().enumerate() {
            let x = ((i % 9) + 1) as u32;
            if line.starts_with(word) {
                tens += x;
            }
            if line.ends_with(word) {
                ones += x;
            }
        }
    }
    println!("{}", u128::from(tens) * 10 + u128::from(ones));
}
