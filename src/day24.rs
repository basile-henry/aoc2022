use alloc::collections::BinaryHeap;
use core::alloc::Allocator;
use core::fmt::Debug;
use core::iter::once;
use std::alloc::System;

use crate::bitset::{U128Set, U32Set};
use crate::hash_set;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day24<A: Allocator + Debug + Copy>(alloc: A, input: &str) -> (u32, u32) {
    let bassin = Bassin::parse(alloc, input);

    let start_x = 0;
    let start_y = 0;
    let end_x = bassin.width - 1;
    let end_y = bassin.height - 1;

    let part1 = dijkstra(
        &bassin,
        Pos {
            time: 0,
            x: start_x,
            y: start_y,
        },
        end_x,
        end_y,
    );

    let back = dijkstra(
        &bassin,
        Pos {
            time: part1,
            x: end_x,
            y: end_y,
        },
        start_x,
        start_y,
    );

    let part2 = dijkstra(
        &bassin,
        Pos {
            time: back,
            x: start_x,
            y: start_y,
        },
        end_x,
        end_y,
    );

    (part1, part2)
}

#[derive(Debug)]
struct Bassin<A: Allocator> {
    columns_up: Vec<U32Set, A>,
    columns_down: Vec<U32Set, A>,
    rows_left: Vec<U128Set, A>,
    rows_right: Vec<U128Set, A>,
    width: u8,
    height: u8,
}

impl<A: Allocator + Copy> Bassin<A> {
    fn parse(alloc: A, input: &str) -> Self {
        let mut columns_up = Vec::with_capacity_in(121, alloc);
        let mut columns_down = Vec::with_capacity_in(121, alloc);
        let mut rows_left = Vec::with_capacity_in(25, alloc);
        let mut rows_right = Vec::with_capacity_in(25, alloc);
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            rows_left.push(U128Set::empty());
            rows_right.push(U128Set::empty());

            let line = line.as_bytes();

            let line = line
                .strip_prefix(&[b'#'])
                .unwrap()
                .strip_suffix(&[b'#'])
                .unwrap();

            for (x, c) in line.iter().enumerate() {
                if columns_up.get(x).is_none() {
                    columns_up.push(U32Set::empty());
                }
                if columns_down.get(x).is_none() {
                    columns_down.push(U32Set::empty());
                }

                match c {
                    b'>' => rows_right[y].insert(x as u8),
                    b'<' => rows_left[y].insert(x as u8),
                    b'v' => columns_down[x].insert(y as u8),
                    b'^' => columns_up[x].insert(y as u8),
                    b'.' => {}
                    b'#' => {}
                    _ => panic!("Unexpected"),
                }
            }

            width = line.len() as u8;
            height = (y + 1) as u8;
        }

        Bassin {
            columns_up,
            columns_down,
            rows_left,
            rows_right,
            width,
            height,
        }
    }

    fn up_blizzard_at(&self, pos: Pos) -> bool {
        let wrap = self.height as isize - 2;
        let blizzard_y = (pos.y as isize - 1 + pos.time as isize).rem_euclid(wrap) + 1;

        self.columns_up[pos.x as usize].contains(blizzard_y as u8)
    }

    fn down_blizzard_at(&self, pos: Pos) -> bool {
        let wrap = self.height as isize - 2;
        let blizzard_y = (pos.y as isize - 1 - pos.time as isize).rem_euclid(wrap) + 1;

        self.columns_down[pos.x as usize].contains(blizzard_y as u8)
    }

    fn right_blizzard_at(&self, pos: Pos) -> bool {
        let wrap = self.width as isize;
        let blizzard_x = (pos.x as isize - pos.time as isize).rem_euclid(wrap);

        self.rows_right[pos.y as usize].contains(blizzard_x as u8)
    }

    fn left_blizzard_at(&self, pos: Pos) -> bool {
        let wrap = self.width as isize;
        let blizzard_x = (pos.x as isize + pos.time as isize).rem_euclid(wrap);

        self.rows_left[pos.y as usize].contains(blizzard_x as u8)
    }

    fn blizzard_at(&self, pos: Pos) -> bool {
        self.up_blizzard_at(pos)
            || self.down_blizzard_at(pos)
            || self.left_blizzard_at(pos)
            || self.right_blizzard_at(pos)
    }

    fn can_move_to(&self, pos: Pos) -> bool {
        if pos.y == 0 {
            return pos.x == 0;
        }
        if pos.y == self.height - 1 {
            return pos.x == self.width - 1;
        }

        !self.blizzard_at(pos)
    }

    #[allow(dead_code)]
    fn draw(&self, time: u32, pos_x: Option<u8>, pos_y: Option<u8>) {
        for y in 0..self.height {
            print!("#");
            if y == 0 || y == self.height - 1 {
                for x in 0..self.width {
                    let pos = Pos { time, x, y };

                    if self.can_move_to(pos) {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
            } else {
                for x in 0..self.width {
                    let pos = Pos { time, x, y };

                    let c = [
                        self.up_blizzard_at(pos),
                        self.down_blizzard_at(pos),
                        self.left_blizzard_at(pos),
                        self.right_blizzard_at(pos),
                    ]
                    .into_iter()
                    .filter(|&x| x)
                    .count();

                    if c > 1 {
                        print!("{c}");
                    } else if self.up_blizzard_at(pos) {
                        print!("^");
                    } else if self.down_blizzard_at(pos) {
                        print!("v");
                    } else if self.left_blizzard_at(pos) {
                        print!("<");
                    } else if self.right_blizzard_at(pos) {
                        print!(">");
                    } else if Some(x) == pos_x && Some(y) == pos_y {
                        print!("E");
                    } else {
                        print!(".");
                    }
                }
            }
            println!("#");
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos {
    time: u32,
    x: u8,
    y: u8,
}

impl Pos {
    fn next(self, width: u8, height: u8) -> impl Iterator<Item = Self> {
        let same_pos = Pos {
            time: self.time + 1,
            ..self
        };

        once(same_pos)
            .chain(
                self.x
                    .checked_add(1)
                    .filter(|&x| x < width)
                    .map(|x| Pos { x, ..same_pos }),
            )
            .chain(
                self.x
                    .checked_sub(1)
                    .filter(|&x| x < width)
                    .map(|x| Pos { x, ..same_pos }),
            )
            .chain(
                self.y
                    .checked_add(1)
                    .filter(|&y| y < height)
                    .map(|y| Pos { y, ..same_pos }),
            )
            .chain(
                self.y
                    .checked_sub(1)
                    .filter(|&y| y < height)
                    .map(|y| Pos { y, ..same_pos }),
            )
    }
}

fn dijkstra<A: Allocator + Copy>(bassin: &Bassin<A>, start: Pos, goal_x: u8, goal_y: u8) -> u32 {
    let cost = |pos: Pos| core::cmp::Reverse(pos.time);

    let mut to_visit = BinaryHeap::with_capacity(1024);
    to_visit.push((cost(start), start));

    let mut seen = hash_set!(128 * 1024, System);
    seen.insert(start);

    while let Some((_, current)) = to_visit.pop() {
        if current.x == goal_x && current.y == goal_y {
            return current.time;
        }

        for next in current.next(bassin.width, bassin.height) {
            if !seen.contains(&next) && bassin.can_move_to(next) {
                to_visit.push((cost(next), next));
                seen.insert(next);
            }
        }
    }

    panic!("Solution not found");
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
"#;
    assert_eq!(day24(&bump, example).0, 18);
    assert_eq!(day24(&bump, example).1, 54);
}