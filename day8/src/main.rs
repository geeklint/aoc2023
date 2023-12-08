use std::collections::HashMap;

fn main1() {
    let input = std::fs::read("input").unwrap();
    let mut lines = input.split(|&c| c == b'\n');
    let left_right = lines.next().unwrap();
    let [] = lines.next().unwrap() else { panic!() };
    let mut nodes = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let letters = line
            .iter()
            .copied()
            .filter(|c| c.is_ascii_uppercase())
            .collect::<Vec<_>>();
        let mut triplets = letters.chunks_exact(3);
        let this: [u8; 3] = triplets.next().unwrap().try_into().unwrap();
        let left: [u8; 3] = triplets.next().unwrap().try_into().unwrap();
        let right: [u8; 3] = triplets.next().unwrap().try_into().unwrap();
        nodes.insert(this, (left, right));
    }
    let mut current = *b"AAA";
    let mut steps = 0;
    for lr in left_right.iter().copied().cycle() {
        let pair = nodes[&current];
        match lr {
            b'L' => current = pair.0,
            b'R' => current = pair.1,
            _ => panic!(),
        }
        steps += 1;
        if current == *b"ZZZ" {
            break;
        }
    }
    dbg!(steps);
}

fn main() {
    let input = std::fs::read("input").unwrap();
    let mut lines = input.split(|&c| c == b'\n');
    let left_right = lines.next().unwrap();
    let [] = lines.next().unwrap() else { panic!() };
    let mut nodes = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let letters = line
            .iter()
            .copied()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<Vec<_>>();
        let mut triplets = letters.chunks_exact(3);
        let this: [u8; 3] = triplets.next().unwrap().try_into().unwrap();
        let left: [u8; 3] = triplets.next().unwrap().try_into().unwrap();
        let right: [u8; 3] = triplets.next().unwrap().try_into().unwrap();
        nodes.insert(this, (left, right));
    }
    let mut current_set = nodes
        .keys()
        .copied()
        .filter(|&[_, _, k]| k == b'A')
        .collect::<Vec<_>>();
    let mut paths = Vec::new();
    for starting_point in current_set {
        let mut steps = 0_u64;
        let mut current = starting_point;
        let mut zs = Vec::new();
        for lr in left_right.iter().cycle() {
            let pair = nodes[&current];
            match lr {
                b'L' => current = pair.0,
                b'R' => current = pair.1,
                _ => panic!(),
            }
            steps += 1;
            if current.ends_with(b"Z") {
                let Some(i) = zs.iter().position(|&(node, cmd, _count, _repeat)| {
                    node == current && std::ptr::eq(lr, cmd)
                }) else {
                    zs.push((current, lr, steps, None));
                    continue;
                };
                let base = zs[i].2;
                let repeat = Some(steps - base);
                for (_, _, count, dest) in zs.iter_mut() {
                    if *count >= base {
                        *dest = repeat;
                    }
                }
                break;
            }
        }
        paths.push(zs);
    }
    dbg!(&paths);
    assert!(paths.iter().all(|zs| zs.len() == 1));
    let mut counts = paths.iter().flatten().map(|z| z.2).collect::<Vec<_>>();
    while !counts.windows(2).all(|w| w[0] == w[1]) {
        let i = counts
            .iter()
            .enumerate()
            .min_by_key(|&(_i, c)| c)
            .unwrap()
            .0;
        counts[i] += paths[i][0].3.unwrap();
    }
    dbg!(&counts);
}
