use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut bottom = 0;
    let first_row = vec![0];
    let mut rows = VecDeque::new();
    rows.push_front(first_row);
    let mut left = 0;
    let mut tops = HashSet::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split_ascii_whitespace();
        let dir = parts.next().unwrap();
        let len: i32 = parts.next().unwrap().parse().unwrap();
        let _color = parts.next().unwrap();
        let mut last_top = -1;
        let [mov_x, mov_y] = match dir {
            "U" => {
                tops.insert([cursor_x, cursor_y]);
                last_top = len - 1;
                [0, 1]
            }
            "D" => {
                last_top = len;
                [0, -1]
            }
            "L" => [-1, 0],
            "R" => [1, 0],
            _ => panic!(),
        };
        for i in 0..len {
            cursor_x += mov_x;
            cursor_y += mov_y;
            if i < last_top {
                tops.insert([cursor_x, cursor_y]);
            }
            left = left.min(cursor_x);
            while cursor_y < bottom {
                rows.push_front(Vec::new());
                bottom -= 1;
            }
            while cursor_y >= (rows.len() as i32) + bottom {
                rows.push_back(Vec::new());
            }
            let row = &mut rows[(cursor_y - bottom) as usize];
            row.push(cursor_x);
        }
    }
    let mut sum = 0;
    for (y, mut row) in rows.into_iter().enumerate().rev() {
        let y = (y as i32) + bottom;
        row.sort();
        row.dedup();
        let mut inside = false;
        let mut row_sum = row.len();
        for _ in left..row[0] {
            print!(".");
        }
        for span in row.windows(2) {
            if tops.contains(&[span[0], y]) {
                inside = !inside;
                print!("T");
            } else {
                print!("#");
            }
            if span[1] - span[0] > 1 && inside {
                row_sum += (span[0] + 1..span[1]).count();
            }
            for _ in span[0] + 1..span[1] {
                if inside {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            //if tops.contains(&[span[1], y]) {
            //    inside = !inside;
            //}
        }
        println!("# ({row_sum})");
        sum += row_sum;
    }
    dbg!(sum);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut left = 0;
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Span<T> {
        start: i64,
        end: i64,
        value: T,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Category {
        Up,
        Down,
        Horizontal,
    }
    let mut rows = vec![];
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split_ascii_whitespace();
        let _dir = parts.next().unwrap();
        let _len: i64 = parts.next().unwrap().parse().unwrap();
        let color = parts
            .next()
            .unwrap()
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(')')
            .unwrap();
        let (body, dir) = color.split_at(5);
        let len = i64::from_str_radix(body, 16).unwrap();
        let ([new_x, new_y], category) = match dir {
            "U" | "3" => ([cursor_x, cursor_y + len], Category::Up),
            "D" | "1" => ([cursor_x, cursor_y - len], Category::Down),
            "L" | "2" => ([cursor_x - len, cursor_y], Category::Horizontal),
            "R" | "0" => ([cursor_x + len, cursor_y], Category::Horizontal),
            _ => panic!(),
        };
        let span_x = Span {
            start: cursor_x.min(new_x),
            end: cursor_x.max(new_x),
            value: category,
        };
        let mut span_y = Span {
            start: cursor_y.min(new_y),
            end: cursor_y.max(new_y),
            value: (),
        };
        cursor_x = new_x;
        cursor_y = new_y;
        left = cursor_x.min(left);
        let (Ok(mut index) | Err(mut index)) = rows.binary_search_by(|row_span: &Span<_>| {
            if row_span.end < span_y.start {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        while span_y.start <= span_y.end {
            match rows.get_mut(index) {
                None => {
                    rows.insert(
                        index,
                        Span {
                            start: span_y.start,
                            end: span_y.end,
                            value: vec![span_x],
                        },
                    );
                    break;
                }
                Some(row_span) if span_y.end < row_span.start => {
                    rows.insert(
                        index,
                        Span {
                            start: span_y.start,
                            end: span_y.end,
                            value: vec![span_x],
                        },
                    );
                    break;
                }
                Some(mut row_span) => {
                    if row_span.start < span_y.start {
                        let new = Span {
                            start: row_span.start,
                            end: span_y.start - 1,
                            value: row_span.value.clone(),
                        };
                        row_span.start = span_y.start;
                        rows.insert(index, new);
                        index += 1;
                        row_span = &mut rows[index];
                    } else if span_y.start < row_span.start {
                        let new = Span {
                            start: span_y.start,
                            end: row_span.start - 1,
                            value: vec![],
                        };
                        rows.insert(index, new);
                        row_span = &mut rows[index];
                    }
                    if span_y.end < row_span.end {
                        let new = Span {
                            start: span_y.start,
                            end: span_y.end,
                            value: row_span.value.clone(),
                        };
                        row_span.start = span_y.end + 1;
                        rows.insert(index, new);
                        row_span = &mut rows[index];
                    }
                    assert_eq!(row_span.start, span_y.start);
                    assert!(row_span.end <= span_y.end);
                    row_span.value.push(span_x);
                    span_y.start = row_span.end + 1;
                    index += 1;
                }
            }
        }
    }
    let mut sum = 0;
    for row_span in rows.iter_mut().rev() {
        eprintln!();
        row_span.value.sort();
        let mut row_sum = 0;
        let row_span_len = row_span.end - row_span.start + 1;
        let first = &row_span.value[0];
        let mut dv = match first.value {
            Category::Up => 1,
            Category::Down => -1,
            Category::Horizontal => panic!(),
        };
        let mut seg_start = first.start;
        let mut inside = false;
        for window in row_span.value.windows(2) {
            if window[0].end + 1 < window[1].start {
                if dv != 0 {
                    inside = !inside;
                    dv = 0;
                }
                let seg_len = window[0].end - seg_start + 1;
                row_sum += dbg!(seg_len);
                seg_start = window[1].start;
                if inside {
                    row_sum += dbg!((window[1].start - 1) - (window[0].end + 1) + 1);
                }
            }
            match window[1].value {
                Category::Up => dv += 1,
                Category::Down => dv -= 1,
                Category::Horizontal => {}
            }
        }
        let seg_end = row_span.value.last().unwrap().end;
        let seg_len = seg_end - seg_start + 1;
        row_sum += dbg!(seg_len);
        println!();
        sum += dbg!(row_sum * row_span_len);
    }
    dbg!(sum);
}
