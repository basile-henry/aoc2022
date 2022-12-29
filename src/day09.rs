use core::alloc::Allocator;
use core::cmp::Ordering;
use core::fmt::Debug;
use core::iter::repeat;

use crate::hash_set;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
pub fn day09<A: Allocator + Debug>(alloc: A, input: &str) -> (usize, usize) {
    let (visited1, visited9, _knots) = fold_many0(
        terminated(Move::parse, newline),
        || {
            let mut visited1 = hash_set!(7000, &alloc);
            visited1.insert((0, 0));
            let mut visited9 = hash_set!(3000, &alloc);
            visited9.insert((0, 0));
            (visited1, visited9, [(0, 0); 10])
        },
        |(mut visited1, mut visited9, mut knots), m| {
            for m in m.steps() {
                knots[0] = m.apply_move(knots[0]);
                let mut prev = knots[0];

                for (i, k) in knots[1..].iter_mut().enumerate() {
                    let needs_to_move = (prev.0 - k.0).abs() > 1 || (prev.1 - k.1).abs() > 1;

                    if needs_to_move {
                        k.0 = match prev.0.cmp(&k.0) {
                            Ordering::Greater => k.0 + 1,
                            Ordering::Less => k.0 - 1,
                            Ordering::Equal => k.0,
                        };

                        k.1 = match prev.1.cmp(&k.1) {
                            Ordering::Greater => k.1 + 1,
                            Ordering::Less => k.1 - 1,
                            Ordering::Equal => k.1,
                        };

                        if i == 0 {
                            visited1.insert(*k);
                        }

                        if i == 8 {
                            visited9.insert(*k);
                        }
                    }

                    prev = *k;
                }
            }

            (visited1, visited9, knots)
        },
    )(input)
    .unwrap()
    .1;

    (visited1.len(), visited9.len())
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

impl Move {
    fn parse(input: &str) -> nom::IResult<&str, Move, ()> {
        alt((
            map(preceded(tag("U "), u8), Move::Up),
            map(preceded(tag("D "), u8), Move::Down),
            map(preceded(tag("L "), u8), Move::Left),
            map(preceded(tag("R "), u8), Move::Right),
        ))(input)
    }

    fn steps(self) -> impl Iterator<Item = Self> {
        match self {
            Move::Up(c) => repeat(Move::Up(1)).take(c as usize),
            Move::Down(c) => repeat(Move::Down(1)).take(c as usize),
            Move::Left(c) => repeat(Move::Left(1)).take(c as usize),
            Move::Right(c) => repeat(Move::Right(1)).take(c as usize),
        }
    }

    fn apply_move(self, (x, y): (i16, i16)) -> (i16, i16) {
        match self {
            Move::Up(c) => (x, y + c as i16),
            Move::Down(c) => (x, y - c as i16),
            Move::Left(c) => (x - c as i16, y),
            Move::Right(c) => (x + c as i16, y),
        }
    }
}

#[test]
fn example1() {
    let bump = bumpalo::Bump::new();
    let example = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;
    assert_eq!(day09(&bump, example).0, 13);
    assert_eq!(day09(&bump, example).1, 1);
}

#[test]
fn example2() {
    let bump = bumpalo::Bump::new();
    let example = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;
    assert_eq!(day09(&bump, example).0, 88);
    assert_eq!(day09(&bump, example).1, 36);
}
