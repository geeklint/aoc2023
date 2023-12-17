fn main1() {
    use std::cmp::Reverse;
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    for ch in &mut input {
        *ch -= b'0';
    }
    let input = input;
    let validate_step = |from: usize, to: usize| {
        (0..input.len()).contains(&to) && (from % width).abs_diff(to % width) <= 1
    };
    let down = width as isize;
    let up = -down;
    let height = input.len() / width;
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Path {
        cost: Reverse<usize>,
        momentum: Reverse<usize>,
        path: Vec<usize>,
    }
    let mut paths = std::collections::BinaryHeap::new();
    let mut edges = std::collections::HashMap::new();
    paths.push(Path {
        path: vec![0],
        cost: Reverse(0),
        momentum: Reverse(0),
    });
    let dest = input.len() - 1;
    let mut dbg_count = 0;
    let final_cost = 'path: loop {
        let path = paths.pop().unwrap();
        dbg_count += 1;
        if dbg_count >= 10 {
            dbg_count = 0;
            println!();
            println!();
            dbg!(&path.path);
            dbg!(path.cost);
            dbg!(path.momentum);
            for y in 0..height {
                for x in 0..width {
                    let i = y * width + x;
                    if path.path.contains(&i) {
                        print!(".");
                    } else {
                        print!("{}", (input[i] + b'0') as char);
                    }
                }
                println!();
            }
        }
        let head: usize = match path.path.as_slice() {
            &[.., prev, head] => {
                edges.insert([prev, head], path.momentum.0);
                head
            }
            &[head] => head,
            [] => panic!(),
        };
        paths.retain(|other| {
            let &[.., a, b] = other.path.as_slice() else {
                return true;
            };
            !path.path.ends_with(&[a, b]) || other.momentum.0 < path.momentum.0
        });
        for mov in [up, -1, 1, down] {
            let next = head.wrapping_add_signed(mov);
            let momentum = if let [.., prev, _] = path.path.as_slice() {
                if next == *prev {
                    continue;
                } else if prev.wrapping_add_signed(mov) == head {
                    path.momentum.0 + 1
                } else {
                    0
                }
            } else {
                1
            };
            if momentum == 3 {
                continue;
            }
            if next == dest {
                break 'path path.cost.0 + usize::from(input[next]);
            }
            if let Some(&mo) = edges.get(&[head, next]) {
                if mo <= momentum {
                    continue;
                }
            }
            if validate_step(head, next) {
                let mut new = path.path.clone();
                new.push(next);
                let cost = path.cost.0 + usize::from(input[next]);
                paths.push(Path {
                    path: new,
                    cost: Reverse(cost),
                    momentum: Reverse(momentum),
                });
            }
        }
    };
    dbg!(final_cost);
}

fn main() {
    use std::cmp::Reverse;
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    for ch in &mut input {
        *ch -= b'0';
    }
    let input = input;
    let down = width as isize;
    let up = -down;
    let height = input.len() / width;
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Path {
        cost: Reverse<usize>,
        path: Vec<usize>,
    }
    impl Path {
        fn debug(&self, width: usize, height: usize, input: &[u8]) {
            println!();
            println!();
            dbg!(&self.path);
            dbg!(self.cost);
            for y in 0..height {
                for x in 0..width {
                    let i = y * width + x;
                    if self.path.contains(&i) {
                        print!(".");
                    } else {
                        print!("{}", (input[i] + b'0') as char);
                    }
                }
                println!();
            }
        }
    }
    let mut paths = std::collections::BinaryHeap::new();
    let mut visited = std::collections::HashSet::new();
    paths.push(Path {
        path: vec![0],
        cost: Reverse(0),
    });
    let dest = input.len() - 1;
    let mut dbg_count = 0;
    let final_cost = loop {
        let path = paths.pop().unwrap();
        dbg_count += 1;
        if dbg_count >= 10000 {
            dbg_count = 0;
            path.debug(width, height, &input);
        }
        let verify_cost = path
            .path
            .windows(2)
            .flat_map(|win| {
                let from = win[0];
                let to = win[1];
                let step_size = if to.abs_diff(from) > 25 { width } else { 1 };
                let (start, end) = if from < to {
                    (from + step_size, to)
                } else {
                    (to, from - step_size)
                };
                (0..)
                    .map(move |s| s * step_size + start)
                    .take_while(move |&i| i <= end)
                    .map(|i| usize::from(input[i]))
            })
            .sum::<usize>();
        assert_eq!(path.cost.0, verify_cost);
        let head: usize = match path.path.as_slice() {
            &[.., prev, head] => {
                let was_horiz = head.abs_diff(prev) < 11;
                visited.insert((head, was_horiz));
                paths.retain(|other| {
                    let &[.., a, b] = other.path.as_slice() else {
                        return true;
                    };
                    let other_horiz = b.abs_diff(a) < 11;
                    b != head || was_horiz != other_horiz
                });
                head
            }
            &[head] => head,
            [] => panic!(),
        };
        if head == dest {
            path.debug(width, height, &input);
            break path.cost.0;
        }
        let sol = (head / width) * width;
        let eol = sol + width;
        for (mov_dir, min, limit) in [
            (up, 0, input.len()),
            (-1, sol, eol),
            (1, sol, eol),
            (down, 0, input.len()),
        ] {
            let is_horiz = (-1..=1).contains(&mov_dir);
            if let &[.., prev, head] = path.path.as_slice() {
                let was_horiz = head.abs_diff(prev) < 11;
                if was_horiz == is_horiz {
                    continue;
                }
            };
            for next in (4..=10)
                .map(|m| head.wrapping_add_signed(m * mov_dir))
                .filter(|n| (min..limit).contains(n))
            {
                if visited.contains(&(next, is_horiz)) {
                    continue;
                }
                let mut cost = path.cost.0;
                let mut substep = head;
                loop {
                    substep = substep.wrapping_add_signed(mov_dir);
                    cost += usize::from(input[substep]);
                    if substep == next {
                        break;
                    }
                }
                let mut new = path.path.clone();
                new.push(next);
                paths.push(Path {
                    path: new,
                    cost: Reverse(cost),
                });
            }
        }
    };
    dbg!(final_cost);
}
