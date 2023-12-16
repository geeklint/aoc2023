fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    let sum: u64 = input
        .trim()
        .split(',')
        .map(|step| holiday_hash(step) as u64)
        .sum();
    dbg!(sum);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    const BOX: Vec<(&str, usize)> = Vec::new();
    let mut boxes = [BOX; 256];
    for step in input.trim().split(',') {
        if let Some(label) = step.strip_suffix('-') {
            let b = &mut boxes[holiday_hash(label) as usize];
            b.retain(|&(existing, _)| existing != label);
        } else {
            let Some((label, lens)) = step.split_once('=') else {
                panic!()
            };
            let lens = lens.parse().unwrap();
            let b = &mut boxes[holiday_hash(label) as usize];
            if let Some((_, slot)) = b.iter_mut().find(|&&mut (existing, _)| existing == label) {
                *slot = lens;
            } else {
                b.push((label, lens));
            }
        }
    }
    let sum: usize = boxes
        .into_iter()
        .enumerate()
        .flat_map(|(bi, bx)| {
            bx.into_iter()
                .enumerate()
                .map(move |(li, (_, lens))| (bi + 1) * (li + 1) * lens)
        })
        .sum();
    dbg!(sum);
}

fn holiday_hash(s: &str) -> u8 {
    s.chars().fold(0_u32, |mut accum, c| {
        accum += c as u32;
        accum *= 17;
        accum &= 0xff;
        accum
    }) as u8
}

#[test]
fn test_hash() {
    assert_eq!(holiday_hash("HASH"), 52);
}
