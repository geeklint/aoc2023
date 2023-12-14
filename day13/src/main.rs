fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut sum = 0;
    for grid in input.trim_end().split("\n\n") {
        println!("{grid}");
        let width = grid.as_bytes().iter().position(|&c| c == b'\n').unwrap();
        let grid: Vec<u8> = grid
            .as_bytes()
            .iter()
            .copied()
            .filter(|&c| c != b'\n')
            .collect();
        let height = grid.len() / width;
        let mut lines = grid.chunks_exact(width).enumerate().peekable();
        let mut cols = (0..width).peekable();
        let column =
            |i: usize| -> Vec<u8> { grid.chunks_exact(width).map(|line| line[i]).collect() };
        'row_pairs: while let (Some((_, top)), Some(&(i, bottom))) = (lines.next(), lines.peek()) {
            if top == bottom {
                let above = i;
                let below = height - i;
                let reflection_size = above.min(below);
                let start = i - reflection_size;
                let end = i + reflection_size;
                let span = &grid[(start * width)..(end * width)];
                let mut span_rows = span.chunks_exact(width);
                loop {
                    let Some(above) = span_rows.next() else { break };
                    let below = span_rows.next_back().unwrap();
                    if above != below {
                        continue 'row_pairs;
                    }
                }
                sum += 100 * dbg!(i);
            }
        }
        'col_pairs: while let (Some(j), Some(&i)) = (cols.next(), cols.peek()) {
            let left = column(j);
            let right = column(i);
            if left == right {
                let num_left = i;
                let num_right = width - i;
                let reflection_size = num_left.min(num_right);
                let start = i - reflection_size;
                let end = i + reflection_size;
                let mut span_cols = (start..end).map(column);
                loop {
                    let Some(left) = span_cols.next() else { break };
                    let right = span_cols.next_back().unwrap();
                    if left != right {
                        continue 'col_pairs;
                    }
                }
                sum += dbg!(i);
            }
        }
    }
    dbg!(sum);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut sum = 0;
    'grid: for grid in input.trim_end().split("\n\n") {
        println!("{grid}");
        let width = grid.as_bytes().iter().position(|&c| c == b'\n').unwrap();
        let grid: Vec<u8> = grid
            .as_bytes()
            .iter()
            .copied()
            .filter(|&c| c != b'\n')
            .collect();
        let original = find_mirror(&grid, width, 0).unwrap();
        dbg!(original);
        for smudge in 0..grid.len() {
            let mut fixed = grid.clone();
            fixed[smudge] = match grid[smudge] {
                b'.' => b'#',
                b'#' => b'.',
                _ => panic!(),
            };
            if let Some(new) = find_mirror(&fixed, width, original) {
                if new != original {
                    sum += new;
                    continue 'grid;
                }
            }
        }
        panic!()
    }
    dbg!(sum);
    fn find_mirror(grid: &[u8], width: usize, ignore: usize) -> Option<usize> {
        let height = grid.len() / width;
        let mut lines = grid.chunks_exact(width).enumerate().peekable();
        let mut cols = (0..width).peekable();
        let column =
            |i: usize| -> Vec<u8> { grid.chunks_exact(width).map(|line| line[i]).collect() };
        //let s = std::str::from_utf8(lines.peek().unwrap().1).unwrap();
        //dbg!(s);
        'row_pairs: while let (Some((_, top)), Some(&(i, bottom))) = (lines.next(), lines.peek()) {
            if top == bottom {
                let above = i;
                let below = height - i;
                let reflection_size = above.min(below);
                let start = i - reflection_size;
                let end = i + reflection_size;
                let span = &grid[(start * width)..(end * width)];
                let mut span_rows = span.chunks_exact(width);
                loop {
                    let Some(above) = span_rows.next() else { break };
                    let below = span_rows.next_back().unwrap();
                    if above != below {
                        continue 'row_pairs;
                    }
                }
                if 100 * i != ignore {
                    return Some(100 * dbg!(i));
                }
            }
        }
        'col_pairs: while let (Some(j), Some(&i)) = (cols.next(), cols.peek()) {
            let left = column(j);
            let right = column(i);
            if left == right {
                let num_left = i;
                let num_right = width - i;
                let reflection_size = num_left.min(num_right);
                let start = i - reflection_size;
                let end = i + reflection_size;
                let mut span_cols = (start..end).map(column);
                loop {
                    let Some(left) = span_cols.next() else { break };
                    let right = span_cols.next_back().unwrap();
                    if left != right {
                        continue 'col_pairs;
                    }
                }
                if i != ignore {
                    return Some(dbg!(i));
                }
            }
        }
        None
    }
}
