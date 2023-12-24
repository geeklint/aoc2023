use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    sync::atomic::AtomicI64,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main1() {
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    let validate_step = |from: usize, to: usize| {
        (0..input.len()).contains(&to) && (from % width).abs_diff(to % width) <= 1
    };
    let starting = input.iter().copied().position(|c| c == b'S').unwrap();
    let mut shadow_clones = vec![starting];
    for _ in 0..64 {
        let mut dests = HashSet::new();
        for sc in shadow_clones.drain(..) {
            let dest = sc - width;
            if validate_step(sc, dest) && input[dest] != b'#' {
                dests.insert(dest);
            }
            let dest = sc - 1;
            if validate_step(sc, dest) && input[dest] != b'#' {
                dests.insert(dest);
            }
            let dest = sc + 1;
            if validate_step(sc, dest) && input[dest] != b'#' {
                dests.insert(dest);
            }
            let dest = sc + width;
            if validate_step(sc, dest) && input[dest] != b'#' {
                dests.insert(dest);
            }
        }
        shadow_clones = dests.into_iter().collect();
    }
    dbg!(shadow_clones.len());
}

fn main() {
    let input_steps = 26501365_i64;
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    let validate_step = |from: usize, to: usize| {
        (0..input.len()).contains(&to) && (from % width).abs_diff(to % width) <= 1
    };
    let starting = input.iter().copied().position(|c| c == b'S').unwrap();
    let mut reachable = HashSet::new();
    reachable.insert(starting);
    let mut shadow_clones = vec![starting];
    while !shadow_clones.is_empty() {
        let mut dests = HashSet::new();
        for sc in shadow_clones.drain(..) {
            dests.extend(
                [sc - width, sc - 1, sc + 1, sc + width]
                    .into_iter()
                    .filter(|&dest| {
                        validate_step(sc, dest) && input[dest] != b'#' && reachable.insert(dest)
                    }),
            );
        }
        shadow_clones = dests.into_iter().collect();
    }
    let distance_between = |a: usize, b: usize| -> usize {
        let mut visited = HashSet::new();
        let mut edges = BinaryHeap::new();
        edges.push((Reverse(0), a));
        if a == b {
            return 0;
        }
        loop {
            let Some((Reverse(cost), to)) = edges.pop() else {
                dbg!(a, b);
                panic!();
            };
            if to == b {
                return cost;
            }
            if visited.insert(to) {
                edges.extend(
                    [to - width, to - 1, to + 1, to + width]
                        .into_iter()
                        .filter(|&next| {
                            validate_step(to, next)
                                && reachable.contains(&next)
                                && !visited.contains(&next)
                        })
                        .map(|next| (Reverse(cost + 1), next)),
                );
            }
        }
    };
    dbg!(width);
    let iwidth = width as i64;
    let height = (input.len() as i64) / iwidth;
    let starting = input.iter().copied().position(|c| c == b'S').unwrap();
    let starting_row = starting as i64 / iwidth;
    let starting_col = starting as i64 % iwidth;
    let dist_map: HashMap<_, _> = [
        0,
        starting_col as usize,
        width - 1,
        starting_row as usize * width,
        starting_row as usize * width + starting_col as usize,
        starting_row as usize * width + width - 1,
        input.len() - width,
        input.len() - width + starting_col as usize,
        input.len() - 1,
    ]
    .into_par_iter()
    .flat_map(|poi| {
        (0..input.len())
            .into_par_iter()
            .filter(|i| reachable.contains(i))
            .map(move |i| (i, poi))
    })
    .map(|(i, poi)| ((i, poi), distance_between(i, poi)))
    .collect();
    let distance_between = |a: usize, b: usize| -> usize {
        let Some(&dist) = dist_map.get(&(a, b)) else {
            dbg!(a, b);
            panic!()
        };
        dist
    };
    dbg!();
    let progress = AtomicI64::new(0);
    let valid_spaces = ((-input_steps)..=input_steps)
        .into_par_iter()
        .map(|rel_y| {
            let mut valid_spaces = 0;
            let x_limit = input_steps - rel_y.abs();
            let y = rel_y + starting_row;
            let (poi_row_start, origin_poi_row_start, poi_y_middle_dist);
            if y < 0 {
                poi_row_start = input.len() - width;
                origin_poi_row_start = 0;
                let chunks = (y.abs() - 1) / height;
                poi_y_middle_dist = if chunks == 0 {
                    1
                } else {
                    (chunks * (height - 1)) + 2
                };
            } else if y >= height {
                poi_row_start = 0;
                origin_poi_row_start = input.len() - width;
                let chunks = (y / height) - 1;
                poi_y_middle_dist = if chunks == 0 {
                    1
                } else {
                    (chunks * (height - 1)) + 2
                };
            } else {
                poi_row_start = starting_row as usize * width;
                origin_poi_row_start = starting_row as usize * width;
                poi_y_middle_dist = 0;
            };
            let row_start = y.rem_euclid(height) as usize * width;
            let mut row = input[row_start..][..width].to_vec();
            for i in (row_start..).take(width) {
                if !reachable.contains(&i) {
                    continue;
                }
                let col = (i as i64) % iwidth;
                let rel_x = col - starting_col;
                let (x_behind, x_ahead) = if rel_x > 0 {
                    (rel_x - iwidth, rel_x)
                } else {
                    (rel_x, rel_x + iwidth)
                };
                let (mut first_x, mut last_x) = (x_behind, x_ahead);
                if first_x < -x_limit && last_x > x_limit {
                    continue;
                } else if first_x < -x_limit {
                    first_x = last_x;
                } else if last_x > x_limit {
                    last_x = first_x;
                }
                first_x -= (x_limit - first_x.abs()) / iwidth * iwidth;
                last_x += (x_limit - last_x) / iwidth * iwidth;
                while first_x < last_x {
                    if (first_x.abs() + rel_y.abs()) % 2 != input_steps % 2 {
                        first_x += iwidth;
                        continue;
                    }
                    let (poi_col, origin_poi_col, poi_x_middle_dist);
                    let x = first_x + starting_col;
                    if x < 0 {
                        poi_col = width - 1;
                        origin_poi_col = 0;
                        let chunks = (x.abs() - 1) / iwidth;
                        poi_x_middle_dist = if chunks == 0 {
                            1
                        } else {
                            (chunks * (iwidth - 1)) + 2
                        };
                    } else if x >= iwidth {
                        poi_col = 0;
                        origin_poi_col = width - 1;
                        let chunks = x / iwidth - 1;
                        poi_x_middle_dist = if chunks == 0 {
                            1
                        } else {
                            (chunks * (iwidth - 1)) + 2
                        };
                    } else {
                        poi_col = starting_col as usize;
                        origin_poi_col = starting_col as usize;
                        poi_x_middle_dist = 0;
                    };
                    let poi = poi_row_start + poi_col;
                    let origin_poi = origin_poi_row_start + origin_poi_col;
                    let dist = (distance_between(i, poi) as i64)
                        + poi_y_middle_dist
                        + poi_x_middle_dist
                        + (distance_between(starting, origin_poi) as i64);
                    if dist <= input_steps {
                        break;
                    }
                    first_x += iwidth;
                }
                while last_x >= first_x {
                    if (last_x.abs() + rel_y.abs()) % 2 != input_steps % 2 {
                        last_x -= iwidth;
                        continue;
                    }
                    let (poi_col, poi_x_dist);
                    let x = last_x + starting_col;
                    if x < 0 {
                        poi_col = width - 1;
                        poi_x_dist = (x.abs() / iwidth * iwidth) + starting_col + 1;
                    } else if x >= iwidth {
                        poi_col = 0;
                        poi_x_dist = (x / iwidth * iwidth) - starting_col;
                    } else {
                        poi_col = starting_col as usize;
                        poi_x_dist = 0;
                    };
                    let poi = poi_row_start + poi_col;
                    let dist = (distance_between(i, poi) as i64) + poi_y_middle_dist + poi_x_dist;
                    if dist <= input_steps {
                        break;
                    }
                    last_x -= iwidth;
                }
                if first_x <= last_x {
                    row[col as usize] = b'O';
                    let pu_spaces = (last_x - first_x) / iwidth + 1;
                    assert_eq!(pu_spaces % 2, 1);
                    let even_spaces = (pu_spaces / 2) + 1;
                    valid_spaces += even_spaces;
                }
            }
            let done = progress.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let old_progress = done * 100 / (2 * input_steps);
            let new_progress = (done + 1) * 100 / (2 * input_steps);
            if done == 0 || old_progress != new_progress {
                println!("{new_progress}%");
            }
            valid_spaces
        })
        .sum::<i64>();
    dbg!(valid_spaces);
}
