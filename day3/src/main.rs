use std::cell::Cell;

fn main1() {
    const HIGH_BIT: u8 = 0b10000000;
    const LOW_BITS: u8 = 0b01111111;
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap() as i32;
    input.retain(|&ch| ch != b'\n');
    //let height = input.len() / width;
    let grid = Cell::from_mut(&mut input[..]).as_slice_of_cells();
    for (i, symbol) in grid.iter().enumerate() {
        match symbol.get() & LOW_BITS {
            b'.' => (),
            b'0'..=b'9' => (),
            _ => {
                let col = (i as i32) % width;
                let x_offsets = if col == 0 {
                    &[0, 1][..]
                } else if col == (width - 1) {
                    &[-1, 0][..]
                } else {
                    &[-1, 0, 1][..]
                };
                for y in [-1, 0, 1] {
                    for x in x_offsets {
                        let offset = y * width + x;
                        let other_i = (i as i32) + offset;
                        let other_i = match usize::try_from(other_i) {
                            Ok(oi) if oi < grid.len() => oi,
                            _ => continue,
                        };
                        grid[other_i].set(grid[other_i].get() | HIGH_BIT);
                    }
                }
            }
        }
    }
    let mut sum: u32 = 0;
    for line in grid.chunks_exact(width as usize) {
        const FLAGGED_0: u8 = b'0' | HIGH_BIT;
        const FLAGGED_9: u8 = b'9' | HIGH_BIT;
        let mut possible_backtrack = true;
        while possible_backtrack {
            possible_backtrack = false;
            let mut windows = line.windows(2);
            while let Some([a, b]) = windows.next() {
                match [a.get(), b.get()] {
                    [FLAGGED_0..=FLAGGED_9, b'0'..=b'9'] => {
                        b.set(b.get() | HIGH_BIT);
                    }
                    [b'0'..=b'9', FLAGGED_0..=FLAGGED_9] => {
                        a.set(a.get() | HIGH_BIT);
                        possible_backtrack = true;
                    }
                    _ => (),
                }
            }
        }
        let mut final_line = line;
        while !final_line.is_empty() {
            let first_digit = final_line
                .iter()
                .position(|c| matches!(c.get(), FLAGGED_0..=FLAGGED_9));
            match first_digit {
                None => break,
                Some(i) => {
                    let remaining = &final_line[i..];
                    let end_of_digit = remaining
                        .iter()
                        .position(|c| !matches!(c.get(), FLAGGED_0..=FLAGGED_9))
                        .unwrap_or(remaining.len());
                    let (digits, rest) = remaining.split_at(end_of_digit);
                    final_line = rest;
                    for digit in digits {
                        digit.set(digit.get() & LOW_BITS);
                    }
                    let number_bytes = digits.iter().map(Cell::get).collect::<Vec<_>>();
                    sum += std::str::from_utf8(&number_bytes)
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                }
            }
        }
    }
    dbg!(sum);
}

fn main() {
    const HIGH_BIT: u8 = 0b10000000;
    const LOW_BITS: u8 = 0b01111111;
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap() as i32;
    input.retain(|&ch| ch != b'\n');
    let grid = Cell::from_mut(&mut input[..]).as_slice_of_cells();
    let mut gears = 0;
    for (i, symbol) in grid.iter().enumerate() {
        match symbol.get() & LOW_BITS {
            b'0'..=b'9' => (),
            b'*' => {
                let col = (i as i32) % width;
                let x_offsets = if col == 0 {
                    &[0, 1][..]
                } else if col == (width - 1) {
                    &[-1, 0][..]
                } else {
                    &[-1, 0, 1][..]
                };
                let mut adj_num_count = 0;
                for y in [-1, 0, 1] {
                    let mut digit = false;
                    for x in x_offsets {
                        let offset = y * width + x;
                        let other_i = (i as i32) + offset;
                        let other_i = match usize::try_from(other_i) {
                            Ok(oi) if oi < grid.len() => oi,
                            _ => continue,
                        };
                        let new_digit = grid[other_i].get().is_ascii_digit();
                        if !digit && new_digit {
                            adj_num_count += 1;
                        }
                        digit = new_digit;
                    }
                }
                if adj_num_count == 2 {
                    gears += 1;
                } else {
                    symbol.set(b'.');
                }
            }
            _ => symbol.set(b'.'),
        }
    }
    dbg!(gears);
    let mut grid = grid;
    let mut sum = 0;
    while gears > 0 {
        let i = grid.iter().position(|c| c.get() == b'*').unwrap();

        let col = (i as i32) % width;
        let x_offsets = if col == 0 {
            &[0, 1][..]
        } else if col == (width - 1) {
            &[-1, 0][..]
        } else {
            &[-1, 0, 1][..]
        };
        for y in [-1, 0, 1] {
            for x in x_offsets {
                let offset = y * width + x;
                let other_i = (i as i32) + offset;
                let other_i = match usize::try_from(other_i) {
                    Ok(oi) if oi < grid.len() => oi,
                    _ => continue,
                };
                grid[other_i].set(grid[other_i].get() | HIGH_BIT);
            }
        }
        gears -= 1;
        let start_prev_row = (i / (width as usize)).saturating_sub(1) * (width as usize);
        grid = &grid[start_prev_row..];
        let mut gear_ratio = 1;
        for line in grid.chunks_exact(width as usize) {
            const FLAGGED_0: u8 = b'0' | HIGH_BIT;
            const FLAGGED_9: u8 = b'9' | HIGH_BIT;
            let mut possible_backtrack = true;
            while possible_backtrack {
                possible_backtrack = false;
                let mut windows = line.windows(2);
                while let Some([a, b]) = windows.next() {
                    match [a.get(), b.get()] {
                        [FLAGGED_0..=FLAGGED_9, b'0'..=b'9'] => {
                            b.set(b.get() | HIGH_BIT);
                        }
                        [b'0'..=b'9', FLAGGED_0..=FLAGGED_9] => {
                            a.set(a.get() | HIGH_BIT);
                            possible_backtrack = true;
                        }
                        _ => (),
                    }
                }
            }
            let mut final_line = line;
            while !final_line.is_empty() {
                let first_digit = final_line
                    .iter()
                    .position(|c| matches!(c.get(), FLAGGED_0..=FLAGGED_9));
                match first_digit {
                    None => break,
                    Some(i) => {
                        let remaining = &final_line[i..];
                        let end_of_digit = remaining
                            .iter()
                            .position(|c| !matches!(c.get(), FLAGGED_0..=FLAGGED_9))
                            .unwrap_or(remaining.len());
                        let (digits, rest) = remaining.split_at(end_of_digit);
                        final_line = rest;
                        for digit in digits {
                            digit.set(digit.get() & LOW_BITS);
                        }
                        assert!(gear_ratio <= 999);
                        let number_bytes = digits.iter().map(Cell::get).collect::<Vec<_>>();
                        gear_ratio *= std::str::from_utf8(&number_bytes)
                            .unwrap()
                            .parse::<u32>()
                            .unwrap();
                    }
                }
            }
        }
        sum += gear_ratio;
    }
    dbg!(sum);
}
