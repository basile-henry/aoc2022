use core::alloc::Allocator;
use core::fmt::Debug;

use crate::hash_map;
use crate::hash_set;
use heapless::binary_heap::Min;
use heapless::BinaryHeap;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day12<A: Allocator + Debug>(alloc: A, input: &str) -> (u16, u16) {
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
                    b'E' => 25,
                    _ => c - b'a',
                })
        }
    };

    // Search in reverse, in order to solve part2 along the way
    let mut to_visit = BinaryHeap::<_, Min, 32>::new();
    to_visit.push((0, end)).unwrap();

    let mut costs = hash_map!(width * width, &alloc);
    costs.insert(end, 0);

    let mut visited = hash_set!(width * width, &alloc);

    let mut part1 = None;
    let mut part2 = None;

    while let Some((cost, current)) = to_visit.pop() {
        visited.insert(current);

        if current == start {
            part1 = Some(cost);
            break;
        }

        if part2.is_none() && altitude(current) == Some(0) {
            part2 = Some(cost);
        }

        let (x, y) = current;
        let x = x as isize;
        let y = y as isize;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if x + dx >= 0 && y + dy >= 0 {
                let n = ((x + dx) as u8, (y + dy) as u8);
                let c = altitude(current).unwrap();
                match altitude(n) {
                    Some(a) if c <= a + 1 => match costs.get(&n) {
                        Some(&prev) if cost + 1 >= prev => {}
                        _ => {
                            to_visit.push((cost + 1, n)).unwrap();
                            costs.insert(n, cost + 1);
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    (part1.unwrap(), part2.unwrap())
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
    assert_eq!(day12(&bump, example).1, 29);
}
