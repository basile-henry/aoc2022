use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use bumpalo::collections::String;
use core::alloc::Allocator;
use core::fmt::Debug;

use bumpalo::Bump;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day05<'bump>(alloc: &'bump Bump, input: &[u8]) -> (&'bump str, &'bump str) {
    let (mut rest, stacks) = Stacks::parse(alloc, input).unwrap();

    let mut stacks1 = Stacks {
        stacks: Vec::new_in(alloc),
    };
    stacks.clone_into(&mut stacks1);

    let mut stacks2 = stacks;

    while let Ok((r, m)) = terminated(Move::parse, newline)(rest) {
        rest = r;
        stacks1.apply1(&m);
        stacks2.apply2(&m);
    }

    let mut part1 = String::new_in(alloc);
    for stack in stacks1.stacks {
        part1.push(char::from_u32(stack[stack.len() - 1] as u32).unwrap());
    }

    let mut part2 = String::new_in(alloc);
    for stack in stacks2.stacks {
        part2.push(char::from_u32(stack[stack.len() - 1] as u32).unwrap());
    }

    (part1.into_bump_str(), part2.into_bump_str())
}

const MAX_STACKS: usize = 10;

#[derive(Debug, Clone)]
struct Stacks<A: Allocator> {
    stacks: Vec<Vec<u8, A>, A>,
}

impl<A: Allocator + Copy> Stacks<A> {
    fn parse(alloc: A, input: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        terminated(
            map(
                fold_many0(
                    terminated(row_parse, line_ending),
                    || Vec::with_capacity_in(MAX_STACKS, alloc),
                    |mut stacks, (row, len)| {
                        for (i, x) in row[..len].iter().enumerate() {
                            if let Some(x) = x {
                                if stacks.len() <= i {
                                    stacks.resize(i + 1, Vec::with_capacity_in(100, alloc));
                                }
                                stacks[i].push(*x);
                            }
                        }

                        stacks
                    },
                ),
                |mut stacks| {
                    for stack in stacks.iter_mut() {
                        stack.reverse();
                    }

                    Stacks { stacks }
                },
            ),
            terminated(take_until("\n"), pair(newline, newline)),
        )(input)
    }

    fn apply1(&mut self, m: &Move) {
        for _ in 0..m.count {
            let x = self.stacks[m.from as usize - 1].pop().unwrap();
            self.stacks[m.to as usize - 1].push(x);
        }
    }

    fn apply2(&mut self, m: &Move) {
        let from = &mut self.stacks[m.from as usize - 1];
        let mut top = from.split_off(from.len() - m.count as usize);
        self.stacks[m.to as usize - 1].append(&mut top);
    }
}

type Row = ([Option<u8>; MAX_STACKS], usize);

fn row_parse(input: &[u8]) -> nom::IResult<&[u8], Row, ()> {
    fold_many0(
        terminated(
            alt((
                map(tag("   "), |_| None),
                map(
                    delimited(char('['), take(1usize), char(']')),
                    |c: &[u8]| Some(c[0]),
                ),
            )),
            alt((char(' '), success(' '))),
        ),
        || -> Row { (Default::default(), 0) },
        |(mut row, ix), elem| {
            row[ix] = elem;
            (row, ix + 1)
        },
    )(input)
}

struct Move {
    count: u32,
    from: u32,
    to: u32,
}

impl Move {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Move, ()> {
        map(
            tuple((
                preceded(tag("move "), u32),
                preceded(tag(" from "), u32),
                preceded(tag(" to "), u32),
            )),
            |(count, from, to)| Move { count, from, to },
        )(input)
    }
}

#[test]
fn both_paths() {
    let bump = bumpalo::Bump::new();
    let example = br#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    assert_eq!(day05(&bump, example).0, "CMZ");
    assert_eq!(day05(&bump, example).1, "MCD");
}
