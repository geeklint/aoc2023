fn main1() {
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    let height = input.len() / width;
    let mut galaxies = Vec::new();
    for (i, ch) in input.into_iter().enumerate() {
        if ch == b'#' {
            let x = i % width;
            let y = i / width;
            galaxies.push([x, y])
        }
    }
    for x in (0..width).rev() {
        if !galaxies.iter().any(|g| g[0] == x) {
            dbg!(x);
            for g in &mut galaxies {
                if g[0] > x {
                    g[0] += 1;
                }
            }
        }
    }
    for y in (0..height).rev() {
        if !galaxies.iter().any(|g| g[1] == y) {
            dbg!(y);
            for g in &mut galaxies {
                if g[1] > y {
                    g[1] += 1;
                }
            }
        }
    }
    let mut sum = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[..i] {
            let horiz = g1[0].abs_diff(g2[0]);
            let vert = g1[1].abs_diff(g2[1]);
            sum += horiz + vert;
        }
    }
    dbg!(sum);
}

fn main() {
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    let height = input.len() / width;
    let mut galaxies = Vec::new();
    for (i, ch) in input.into_iter().enumerate() {
        if ch == b'#' {
            let x = i % width;
            let y = i / width;
            galaxies.push([x, y])
        }
    }
    for x in (0..width).rev() {
        if !galaxies.iter().any(|g| g[0] == x) {
            dbg!(x);
            for g in &mut galaxies {
                if g[0] > x {
                    g[0] += 1000000 - 1;
                }
            }
        }
    }
    for y in (0..height).rev() {
        if !galaxies.iter().any(|g| g[1] == y) {
            dbg!(y);
            for g in &mut galaxies {
                if g[1] > y {
                    g[1] += 1000000 - 1;
                }
            }
        }
    }
    let mut sum = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[..i] {
            let horiz = g1[0].abs_diff(g2[0]);
            let vert = g1[1].abs_diff(g2[1]);
            sum += horiz + vert;
        }
    }
    dbg!(sum);
}
