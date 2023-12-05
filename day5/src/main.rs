fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut sections = input.split("\n\n");
    let seed_section = sections.next().unwrap();
    let seeds_str = seed_section.strip_prefix("seeds: ").unwrap();
    let mut incoming_values: Vec<u64> = seeds_str
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect();
    let mut outgoing_values = Vec::new();
    let mut value_type = "seed";
    for section in sections {
        let mut lines = section.lines();
        let header = lines.next().unwrap();
        let map_kind = header.strip_suffix(" map:").unwrap();
        dbg!(map_kind);
        let (from, to) = map_kind.split_once("-to-").unwrap();
        assert_eq!(value_type, from);
        value_type = to;
        for line in lines {
            let mut numbers = line.split_whitespace();
            let dest_start: u64 = numbers.next().unwrap().parse().unwrap();
            let src_start: u64 = numbers.next().unwrap().parse().unwrap();
            let len: u64 = numbers.next().unwrap().parse().unwrap();
            assert!(numbers.next().is_none());
            let src_range = (src_start)..(src_start + len);
            incoming_values.retain(|v| {
                if src_range.contains(v) {
                    dbg!(v);
                    outgoing_values.push(dbg!(v - src_start + dest_start));
                    false
                } else {
                    true
                }
            });
        }
        outgoing_values.append(&mut incoming_values);
        std::mem::swap(&mut incoming_values, &mut outgoing_values);
    }
    dbg!(incoming_values.into_iter().min());
}

fn main() {
    use std::cell::Cell;
    let input = std::fs::read_to_string("input").unwrap();
    let mut sections = input.split("\n\n");
    let seed_section = sections.next().unwrap();
    let seeds_str = seed_section.strip_prefix("seeds: ").unwrap();
    let seeds_nums: Vec<u64> = seeds_str
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect();
    let mut incoming_values = Vec::new();
    for pair in seeds_nums.chunks_exact(2) {
        if let [start, len] = *pair {
            incoming_values.push(start..(start + len));
        } else {
            panic!()
        }
    }
    incoming_values.sort_by(|a, b| a.clone().cmp(b.clone()));
    let mut outgoing_values = Vec::new();
    let mut value_type = "seed";
    for section in sections {
        let mut lines = section.lines();
        let header = lines.next().unwrap();
        let map_kind = header.strip_suffix(" map:").unwrap();
        dbg!(&incoming_values);
        dbg!(map_kind);
        let (from, to) = map_kind.split_once("-to-").unwrap();
        assert_eq!(value_type, from);
        value_type = to;
        for line in lines {
            let mut numbers = line.split_whitespace();
            let dest_start: u64 = numbers.next().unwrap().parse().unwrap();
            let src_start: u64 = numbers.next().unwrap().parse().unwrap();
            let len: u64 = numbers.next().unwrap().parse().unwrap();
            assert!(numbers.next().is_none());
            let src_range = (src_start)..(src_start + len);
            let mut remaining = Vec::new();
            for v_range in incoming_values {
                if src_range.start < v_range.end && src_range.end > v_range.start {
                    let overlap_start = src_range.start.max(v_range.start);
                    let overlap_end = src_range.end.min(v_range.end);
                    let before = v_range.start..overlap_start;
                    if !before.is_empty() {
                        remaining.push(before);
                    }
                    let after = overlap_end..v_range.end;
                    if !after.is_empty() {
                        remaining.push(after);
                    }
                    let start = overlap_start - src_start + dest_start;
                    let end = overlap_end - src_start + dest_start;
                    outgoing_values.push(start..end);
                } else {
                    remaining.push(v_range);
                }
            }
            incoming_values = remaining;
        }
        outgoing_values.append(&mut incoming_values);
        std::mem::swap(&mut incoming_values, &mut outgoing_values);
        /*
        incoming_values.sort_by(|a, b| a.clone().cmp(b.clone()));
        for adj in Cell::from_mut(&mut incoming_values[..])
            .as_slice_of_cells()
            .windows(2)
        {
            if let [left_cell, right_cell] = adj {
                let left = left_cell.take();
                let right = right_cell.take();
                if left.end > right.start {
                    panic!()
                } else if left.end == right.start {
                    left_cell.set(left.start..right.end);
                }
            } else {
                panic!()
            }
        }
        incoming_values.retain(|rng| !rng.is_empty());
        */
    }
    incoming_values.sort_by(|a, b| a.clone().cmp(b.clone()));
    dbg!(incoming_values[0].start);
}
