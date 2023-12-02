fn main1() {
    let input = std::fs::read("input").unwrap();
    let mut result = 0;
    for line in input.split(|&ch| ch == b'\n') {
        if line.is_empty() {
            continue;
        }
        let colon = line.iter().position(|&ch| ch == b':').unwrap();
        let (head, body) = line.split_at(colon);
        let body = &body[1..];
        let mut possible = true;
        for game in body.split(|&ch| ch == b';') {
            let (mut red, mut blue, mut green) = (0, 0, 0);
            for mut cubes in game.split(|&ch| ch == b',') {
                while cubes.starts_with(&[32]) {
                    cubes = &cubes[1..];
                }
                let space = cubes.iter().position(|&ch| ch == b' ').unwrap();
                let (count, color) = cubes.split_at(space);
                let dest = match &color[1..] {
                    b"red" => &mut red,
                    b"blue" => &mut blue,
                    b"green" => &mut green,
                    _ => panic!("unknown color {color:?}"),
                };
                *dest += std::str::from_utf8(count).unwrap().parse::<u32>().unwrap();
            }
            dbg!((red, blue, green));
            if red > 12 || blue > 14 || green > 13 {
                possible = false;
            }
        }
        if possible {
            let game_id: u32 = head
                .strip_prefix(b"Game ")
                .and_then(|d| std::str::from_utf8(d).ok())
                .and_then(|s| s.parse().ok())
                .unwrap();
            result += game_id;
        }
    }
    dbg!(result);
}

fn main() {
    let input = std::fs::read("input").unwrap();
    let mut result = 0;
    for line in input.split(|&ch| ch == b'\n') {
        if line.is_empty() {
            continue;
        }
        let colon = line.iter().position(|&ch| ch == b':').unwrap();
        let (head, body) = line.split_at(colon);
        let body = &body[1..];
        let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);
        for game in body.split(|&ch| ch == b';') {
            let (mut red, mut blue, mut green) = (0, 0, 0);
            for mut cubes in game.split(|&ch| ch == b',') {
                while cubes.starts_with(&[32]) {
                    cubes = &cubes[1..];
                }
                let space = cubes.iter().position(|&ch| ch == b' ').unwrap();
                let (count, color) = cubes.split_at(space);
                let dest = match &color[1..] {
                    b"red" => &mut red,
                    b"blue" => &mut blue,
                    b"green" => &mut green,
                    _ => panic!("unknown color {color:?}"),
                };
                *dest += std::str::from_utf8(count).unwrap().parse::<u32>().unwrap();
            }
            max_red = max_red.max(red);
            max_green = max_green.max(green);
            max_blue = max_blue.max(blue);
        }
        let power = max_red * max_green * max_blue;
        result += power;
    }
    dbg!(result);
}
