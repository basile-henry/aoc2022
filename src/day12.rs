use core::alloc::Allocator;
use core::fmt::Debug;

use crate::hash_map;
use crate::hash_set;
use heapless::binary_heap::Min;
use heapless::BinaryHeap;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day12<A: Allocator + Debug>(alloc: A, input: &str) -> (usize, usize) {
    let input = input.as_bytes();

    let mut width = None;
    let mut start = None;
    let mut end = None;

    for (y, line) in input.split(|c| *c == b'\n').enumerate() {
        if width.is_none() {
            width = Some(line.len())
        }

        for (x, c) in line.iter().enumerate() {
            match c {
                b'S' => start = Some((x as u8, y as u8)),
                b'E' => end = Some((x as u8, y as u8)),
                _ => {}
            }
        }
    }

    let width = width.unwrap();
    let start = start.unwrap();
    let end = end.unwrap();

    let altitude = |(x, y): (u8, u8)| {
        if x as usize >= width {
            None
        } else {
            input
                .get(y as usize * (width + 1) + x as usize)
                .map(|c| match c {
                    b'S' => 0,
                    b'E' => 26,
                    _ => c - b'a',
                })
        }
    };

    let mut to_visit = BinaryHeap::<_, Min, 64>::new();
    to_visit.push((1, start)).unwrap();

    let mut costs = hash_map!(&alloc);
    costs.insert(start, vec![start]);

    let mut visited = hash_set!(&alloc);

    let mut path = None;
    let mut part1 = None;

    while let Some((cost, current)) = to_visit.pop() {
        visited.insert(current);

        if current == end {
            path = costs.get(&current);
            part1 = Some(costs.get(&current).unwrap().len() - 1);
            break;
        }

        let path = costs.get(&current).unwrap().clone();
        // let cost = path.len();
        let (x, y) = current;
        let x = x as isize;
        let y = y as isize;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if x + dx >= 0 && y + dy >= 0 {
                let n = ((x + dx) as u8, (y + dy) as u8);
                let c = altitude(current).unwrap();
                match altitude(n) {
                    Some(a) if a <= c + 1 => match costs.get(&n).map(|v| v.len()) {
                        Some(prev) if cost + 1 >= prev => {}
                        _ => {
                            to_visit.push((cost + 1, n)).unwrap();
                            let mut path = path.clone();
                            path.push(n);
                            costs.insert(n, path);
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    for (i, c) in input.iter().enumerate() {
        let x = i % (width + 1);
        let y = i / (width + 1);
        let c = char::from_u32(*c as u32).unwrap();

        if path.unwrap().iter().any(|&p| p == (x as u8, y as u8)) {
            print!("\x1b[31m{c}\x1b[0m");
        } else if visited.contains(&(x as u8, y as u8)) {
            print!("\x1b[32m{c}\x1b[0m");
        } else {
            print!("{c}");
        }
    }

    (part1.unwrap(), 0)
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;
    assert_eq!(day12(&bump, example).0, 31);
    assert_eq!(day12(&bump, example).1, 0);
}
