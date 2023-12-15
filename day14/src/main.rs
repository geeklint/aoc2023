fn main1() {
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    let mut moved = true;
    while moved {
        let mut rows = input.chunks_exact_mut(width).peekable();
        moved = false;
        while let (Some(upper), Some(lower)) = (rows.next(), rows.peek_mut()) {
            for (maybe_slot, maybe_rock) in upper.iter_mut().zip(lower.iter_mut()) {
                if *maybe_slot == b'.' && *maybe_rock == b'O' {
                    std::mem::swap(maybe_slot, maybe_rock);
                    moved = true;
                }
            }
        }
    }
    let height = input.len() / width;
    let mut load = height;
    let mut sum = 0;
    for row in input.chunks_exact(width) {
        let s = std::str::from_utf8(row).unwrap();
        println!("{s}");
        let rocks = row.iter().copied().filter(|&ch| ch == b'O').count();
        sum += rocks * load;
        load -= 1;
    }
    dbg!(sum);
}

fn main() {
    use std::hash::{Hash, Hasher};
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    let height = input.len() / width;
    let mut current = input.clone();
    let mut prev = Vec::new();
    let (first, second) = loop {
        let (first, second) = loop {
            println!();
            println!();
            for row in current.chunks_exact(width) {
                let s = std::str::from_utf8(row).unwrap();
                println!("{s}");
            }
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            current.hash(&mut hasher);
            let hash = hasher.finish();
            let found_match = prev.iter().position(|&o| o == hash);
            let second = prev.len();
            prev.push(hash);
            if let Some(first) = found_match {
                break (first, second);
            }
            spin(&mut current, width);
        };
        dbg!(first, second);
        let mut check_current = input.clone();
        for _ in 0..first {
            spin(&mut check_current, width);
        }
        let first_state = check_current.clone();
        for _ in first..second {
            spin(&mut check_current, width);
        }
        if check_current == first_state {
            break (first, second);
        }
    };
    let loop_len = second - first;
    let spins_to_sim = 1_000_000_000 - first;
    let equiv = first + (spins_to_sim % loop_len);
    let mut final_state = input.clone();
    for _ in 0..equiv {
        spin(&mut final_state, width);
    }
    let mut load = height;
    let mut sum = 0;
    for row in final_state.chunks_exact(width) {
        let s = std::str::from_utf8(row).unwrap();
        println!("{s}");
        let rocks = row.iter().copied().filter(|&ch| ch == b'O').count();
        sum += rocks * load;
        load -= 1;
    }
    dbg!(sum);
    fn spin(slice: &mut [u8], width: usize) {
        north(slice, width);
        horizontal(slice, width, 1);
        south(slice, width);
        horizontal(slice, width, 0);
    }
    fn north(slice: &mut [u8], width: usize) {
        let mut moved = true;
        while moved {
            let mut rows = slice.chunks_exact_mut(width).peekable();
            moved = false;
            while let (Some(upper), Some(lower)) = (rows.next(), rows.peek_mut()) {
                for (maybe_slot, maybe_rock) in upper.iter_mut().zip(lower.iter_mut()) {
                    if *maybe_slot == b'.' && *maybe_rock == b'O' {
                        std::mem::swap(maybe_slot, maybe_rock);
                        moved = true;
                    }
                }
            }
        }
    }
    fn south(slice: &mut [u8], width: usize) {
        let mut moved = true;
        while moved {
            let mut rows = slice.chunks_exact_mut(width).rev().peekable();
            moved = false;
            while let (Some(upper), Some(lower)) = (rows.next(), rows.peek_mut()) {
                for (maybe_slot, maybe_rock) in upper.iter_mut().zip(lower.iter_mut()) {
                    if *maybe_slot == b'.' && *maybe_rock == b'O' {
                        std::mem::swap(maybe_slot, maybe_rock);
                        moved = true;
                    }
                }
            }
        }
    }
    fn horizontal(slice: &mut [u8], width: usize, src: usize) {
        for row in slice.chunks_exact_mut(width) {
            let row = std::cell::Cell::from_mut(row).as_slice_of_cells();
            let mut moved = true;
            while moved {
                moved = false;
                for pair in row.windows(2) {
                    if pair[src].get() == b'O' && pair[1 - src].get() == b'.' {
                        pair[src].swap(&pair[1 - src]);
                        moved = true;
                    }
                }
            }
        }
    }
}
