fn main1() {
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap() as i32;
    input.retain(|&ch| ch != b'\n');
    let start = input.iter().position(|&ch| ch == b'S').unwrap() as i32;

    fn offsets(ch: u8, width: i32) -> [i32; 2] {
        match ch {
            b'|' => [-width, width],
            b'-' => [-1, 1],
            b'L' => [-width, 1],
            b'J' => [-width, -1],
            b'7' => [-1, width],
            b'F' => [1, width],
            _ => panic!(),
        }
    }

    let first_step = if matches!(input[(start - width) as usize], b'|' | b'7' | b'F') {
        start - width
    } else if matches!(input[(start - 1) as usize], b'-' | b'L' | b'F') {
        start - 1
    } else if matches!(input[(start + 1) as usize], b'-' | b'J' | b'7') {
        start + 1
    } else if matches!(input[(start + width) as usize], b'|' | b'L' | b'J') {
        start + width
    } else {
        panic!()
    };
    let mut path = vec![start];
    let mut prev = start;
    let mut step = first_step;
    while step != start {
        path.push(step);
        dbg!(input[step as usize] as char);
        let next = offsets(input[step as usize], width)
            .into_iter()
            .map(|off| step + off)
            .find(|&pos| pos != prev)
            .unwrap();
        prev = step;
        step = next;
    }
    let half_len = path.len() / 2;
    dbg!(half_len);
}

fn main() {
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap() as i32;
    input.retain(|&ch| ch != b'\n');
    let start = input.iter().position(|&ch| ch == b'S').unwrap() as i32;

    fn offsets(ch: u8, width: i32) -> [i32; 2] {
        match ch {
            b'|' => [-width, width],
            b'-' => [-1, 1],
            b'L' => [-width, 1],
            b'J' => [-width, -1],
            b'7' => [-1, width],
            b'F' => [1, width],
            _ => panic!(),
        }
    }

    let first_step = if matches!(input[(start - width) as usize], b'|' | b'7' | b'F') {
        start - width
    } else if matches!(input[(start - 1) as usize], b'-' | b'L' | b'F') {
        start - 1
    } else if matches!(input[(start + 1) as usize], b'-' | b'J' | b'7') {
        start + 1
    } else if matches!(input[(start + width) as usize], b'|' | b'L' | b'J') {
        start + width
    } else {
        panic!()
    };
    let mut path = vec![start];
    let mut prev = start;
    let mut step = first_step;
    while step != start {
        path.push(step);
        dbg!(input[step as usize] as char);
        let next = offsets(input[step as usize], width)
            .into_iter()
            .map(|off| step + off)
            .find(|&pos| pos != prev)
            .unwrap();
        prev = step;
        step = next;
    }
    let min = path.iter().copied().min().unwrap();
    assert_eq!(input[min as usize], b'F');
    let min_i = path.iter().position(|&i| i == min).unwrap();
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Outside {
        Left,
        Right,
    }
    let outside = if path[min_i - 1] == min + 1 {
        Outside::Right
    } else {
        Outside::Left
    };
    let mut prev = start;
    let mut inside = Vec::new();
    for step in path.iter().copied().skip(1) {
        let adj: [i32; 2];
        let (left, right);
        match input[step as usize] {
            b'|' => {
                adj = [step - 1, step + 1];
                if prev > step {
                    left = &adj[0..=0];
                    right = &adj[1..=1];
                } else {
                    left = &adj[1..=1];
                    right = &adj[0..=0];
                }
            }
            b'-' => {
                adj = [step - width, step + width];
                if prev < step {
                    left = &adj[0..=0];
                    right = &adj[1..=1];
                } else {
                    left = &adj[1..=1];
                    right = &adj[0..=0];
                }
            }
            b'L' => {
                adj = [step - 1, step + width];
                if prev < step {
                    right = &adj[..];
                    left = &[];
                } else {
                    left = &adj[..];
                    right = &[];
                }
            }
            b'J' => {
                adj = [step + 1, step + width];
                if prev == step - 1 {
                    right = &adj[..];
                    left = &[];
                } else {
                    left = &adj[..];
                    right = &[];
                }
            }
            b'7' => {
                adj = [step - width, step + 1];
                if prev == step - 1 {
                    left = &adj[..];
                    right = &[];
                } else {
                    right = &adj[..];
                    left = &[];
                }
            }
            b'F' => {
                adj = [step - width, step - 1];
                if prev == step + 1 {
                    right = &adj[..];
                    left = &[];
                } else {
                    left = &adj[..];
                    right = &[];
                }
            }
            _ => continue,
        }
        prev = step;
        let maybe_inside = match outside {
            Outside::Left => right,
            Outside::Right => left,
        };
        for &may_in in maybe_inside {
            if !path.contains(&may_in) && may_in >= 0 && (may_in as usize) < input.len() {
                inside.push(may_in);
            }
        }
    }
    inside.sort();
    inside.dedup();
    let mut i = 0;
    while i < inside.len() {
        let pos = inside[i];
        for off in [-width, -1, 1, width] {
            let adj = pos + off;
            if adj < 0
                || (adj as usize) >= input.len()
                || path.contains(&adj)
                || inside.contains(&adj)
            {
                continue;
            }
            inside.push(adj);
        }
        i += 1;
    }
    dbg!(&inside);
    dbg!(inside.len());
}

/// an alternitive solution I had the idea for after solving it the first time
fn main_alt() {
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap() as i32;
    input.retain(|&ch| ch != b'\n');
    let start = input.iter().position(|&ch| ch == b'S').unwrap() as i32;

    fn offsets(ch: u8, width: i32) -> [i32; 2] {
        match ch {
            b'|' => [-width, width],
            b'-' => [-1, 1],
            b'L' => [-width, 1],
            b'J' => [-width, -1],
            b'7' => [-1, width],
            b'F' => [1, width],
            _ => panic!(),
        }
    }

    let first_step = if matches!(input[(start - width) as usize], b'|' | b'7' | b'F') {
        start - width
    } else if matches!(input[(start - 1) as usize], b'-' | b'L' | b'F') {
        start - 1
    } else if matches!(input[(start + 1) as usize], b'-' | b'J' | b'7') {
        start + 1
    } else if matches!(input[(start + width) as usize], b'|' | b'L' | b'J') {
        start + width
    } else {
        panic!()
    };
    let mut path = vec![start];
    let mut prev = start;
    let mut step = first_step;
    while step != start {
        path.push(step);
        let next = offsets(input[step as usize], width)
            .into_iter()
            .map(|off| step + off)
            .find(|&pos| pos != prev)
            .unwrap();
        prev = step;
        step = next;
    }
    let mut sum = 0;
    let min = path.iter().copied().min().unwrap();
    let max = path.iter().copied().max().unwrap();
    let tops = if first_step == start - width {
        &[b'J', b'L', b'|', b'S'][..]
    } else {
        &[b'J', b'L', b'|'][..]
    };
    for i in min..max {
        if path.contains(&i) {
            continue;
        }
        let eol = ((i / width) + 1) * width;
        let intersects = (i..eol)
            .filter(|&j| tops.contains(&input[j as usize]) && path.contains(&j))
            .count();
        if intersects % 2 == 1 {
            input[i as usize] = b'I';
            sum += 1;
        }
    }
    dbg!(sum);
}
