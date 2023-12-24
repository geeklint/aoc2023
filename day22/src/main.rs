use std::{
    cmp::Ordering,
    collections::HashSet,
    ops::{Range, RangeInclusive},
};

fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    struct Brick {
        name: usize,
        x: RangeInclusive<usize>,
        y: RangeInclusive<usize>,
        z: RangeInclusive<usize>,
        can_be_removed: bool,
    }
    let mut bricks = vec![];
    let mut area_width = 0;
    let mut area_depth = 0;
    for (name, line) in input.lines().enumerate() {
        let (start, end) = line.split_once('~').unwrap();
        let mut values = start.split(',');
        let x0: usize = values.next().unwrap().parse().unwrap();
        let y0: usize = values.next().unwrap().parse().unwrap();
        let z0: usize = values.next().unwrap().parse().unwrap();
        let mut values = end.split(',');
        let x1: usize = values.next().unwrap().parse().unwrap();
        let y1: usize = values.next().unwrap().parse().unwrap();
        let z1: usize = values.next().unwrap().parse().unwrap();
        let xn = x0.min(x1);
        let yn = y0.min(y1);
        let zn = z0.min(z1);
        let xx = x0.max(x1);
        let yx = y0.max(y1);
        let zx = z0.max(z1);
        area_width = area_width.max(xx + 1);
        area_depth = area_depth.max(yx + 1);
        bricks.push(Brick {
            name,
            x: xn..=xx,
            y: yn..=yx,
            z: zn..=zx,
            can_be_removed: true,
        });
    }
    bricks.sort_by_key(|brick| *brick.z.start());
    #[derive(Clone, Copy)]
    struct Square {
        top: usize,
        top_brick: usize,
    }
    let row_template = vec![
        Square {
            top: 0,
            top_brick: usize::MAX
        };
        area_width
    ];
    let mut rows = vec![row_template; area_depth];
    for i in 0..bricks.len() {
        let brick = &mut bricks[i];
        let mut supported_by = vec![];
        let mut found_top = 0;
        for row in &rows[brick.y.clone()] {
            for square in &row[brick.x.clone()] {
                match square.top.cmp(&found_top) {
                    Ordering::Greater => {
                        found_top = square.top;
                        supported_by = vec![square.top_brick];
                    }
                    Ordering::Equal => {
                        if square.top_brick != usize::MAX {
                            supported_by.push(square.top_brick);
                        }
                    }
                    Ordering::Less => {}
                }
            }
        }
        let to_fall = brick.z.start() - found_top - 1;
        let new_top = brick.z.end() - to_fall;
        for row in &mut rows[brick.y.clone()] {
            for square in &mut row[brick.x.clone()] {
                square.top = new_top;
                square.top_brick = i;
            }
        }
        supported_by.dedup();
        if supported_by.len() == 1 {
            assert!(supported_by[0] < i);
            bricks[supported_by[0]].can_be_removed = false;
        }
    }
    let can_be_removed = bricks.iter().filter(|brick| brick.can_be_removed).count();
    dbg!(can_be_removed);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    struct Brick {
        name: usize,
        x: RangeInclusive<usize>,
        y: RangeInclusive<usize>,
        z: RangeInclusive<usize>,
        supported_by: HashSet<usize>,
        supporting: HashSet<usize>,
    }
    let mut bricks = vec![];
    let mut area_width = 0;
    let mut area_depth = 0;
    for (name, line) in input.lines().enumerate() {
        let (start, end) = line.split_once('~').unwrap();
        let mut values = start.split(',');
        let x0: usize = values.next().unwrap().parse().unwrap();
        let y0: usize = values.next().unwrap().parse().unwrap();
        let z0: usize = values.next().unwrap().parse().unwrap();
        let mut values = end.split(',');
        let x1: usize = values.next().unwrap().parse().unwrap();
        let y1: usize = values.next().unwrap().parse().unwrap();
        let z1: usize = values.next().unwrap().parse().unwrap();
        let xn = x0.min(x1);
        let yn = y0.min(y1);
        let zn = z0.min(z1);
        let xx = x0.max(x1);
        let yx = y0.max(y1);
        let zx = z0.max(z1);
        area_width = area_width.max(xx + 1);
        area_depth = area_depth.max(yx + 1);
        bricks.push(Brick {
            name,
            x: xn..=xx,
            y: yn..=yx,
            z: zn..=zx,
            supported_by: HashSet::new(),
            supporting: HashSet::new(),
        });
    }
    bricks.sort_by_key(|brick| *brick.z.start());
    #[derive(Clone, Copy)]
    struct Square {
        top: usize,
        top_brick: usize,
    }
    let row_template = vec![
        Square {
            top: 0,
            top_brick: usize::MAX
        };
        area_width
    ];
    let mut rows = vec![row_template; area_depth];
    for (i, brick) in bricks.iter_mut().enumerate() {
        let mut supported_by = HashSet::new();
        let mut found_top = 0;
        for row in &rows[brick.y.clone()] {
            for square in &row[brick.x.clone()] {
                match square.top.cmp(&found_top) {
                    Ordering::Greater => {
                        found_top = square.top;
                        supported_by.clear();
                        supported_by.insert(square.top_brick);
                    }
                    Ordering::Equal => {
                        if square.top_brick != usize::MAX {
                            supported_by.insert(square.top_brick);
                        }
                    }
                    Ordering::Less => {}
                }
            }
        }
        let to_fall = brick.z.start() - found_top - 1;
        let new_top = brick.z.end() - to_fall;
        for row in &mut rows[brick.y.clone()] {
            for square in &mut row[brick.x.clone()] {
                square.top = new_top;
                square.top_brick = i;
            }
        }
        brick.supported_by = supported_by;
    }
    for i in 0..bricks.len() {
        let mut foundation = HashSet::new();
        foundation.insert(i);
        for (j, other) in bricks.iter().enumerate().skip(i + 1) {
            if !other.supported_by.is_empty() && other.supported_by.is_subset(&foundation) {
                foundation.insert(j);
            }
        }
        foundation.remove(&i);
        bricks[i].supporting = foundation;
    }
    // bricks.sort_by_key(|brick| brick.name);
    // for brick in bricks {
    //     let name = (brick.name as u8 + b'A') as char;
    //     let supporting = brick.supporting.len();
    //     println!("{name} {supporting}");
    // }
    let sum: usize = bricks.iter().map(|brick| brick.supporting.len()).sum();
    dbg!(sum);
}
