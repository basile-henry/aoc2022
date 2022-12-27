use alloc::vec::Vec;
use core::alloc::Allocator;
use core::cmp::Ordering;
use core::fmt::Debug;

use nom::branch::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

use crate::nom_extra::separated_fold_many0;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day13<A: Allocator + Debug + Copy>(alloc: A, input: &str) -> (usize, usize) {
    let (part1, _, mut packets) = fold_many0(
        terminated(|i| parse_pair(alloc, i), alt((eof, line_ending))),
        || (0, 1, Vec::with_capacity_in(300, alloc)),
        |(mut sum, ix, mut packets), (left, right)| {
            if Packet::right_order(&left, &right).unwrap() {
                sum += ix;
            }

            packets.push(left);
            packets.push(right);

            (sum, ix + 1, packets)
        },
    )(input)
    .unwrap()
    .1;

    let singleton = |x| Box::new_in([x], alloc);

    let a = Packet::List(singleton(Packet::List(singleton(Packet::Num(2)))));
    let b = Packet::List(singleton(Packet::List(singleton(Packet::Num(6)))));

    packets.push(a.clone());
    packets.push(b.clone());

    let packet_ordering = |l: &Packet<A>, r: &Packet<A>| match Packet::right_order(l, r) {
        None => Ordering::Equal,
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
    };

    packets.sort_unstable_by(&packet_ordering);

    let a_find = move |x| packet_ordering(x, &a);
    let b_find = move |x| packet_ordering(x, &b);

    let a = 1 + packets.binary_search_by(a_find).unwrap();
    let b = 1 + packets.binary_search_by(b_find).unwrap();

    let part2 = a * b;

    (part1, part2)
}

#[derive(Debug, Clone)]
enum Packet<A: Allocator> {
    Num(u8),
    List(Box<[Packet<A>], A>),
}

impl<A: Allocator + Copy> Packet<A> {
    fn parse(alloc: A, input: &str) -> nom::IResult<&str, Self, ()> {
        alt((
            map(u8, Packet::Num),
            preceded(
                char('['),
                terminated(
                    map(
                        separated_fold_many0(
                            char(','),
                            |i| Packet::parse(alloc, i),
                            || Vec::new_in(alloc),
                            |mut v, e| {
                                v.push(e);
                                v
                            },
                        ),
                        |v| Packet::List(v.into_boxed_slice()),
                    ),
                    char(']'),
                ),
            ),
        ))(input)
    }

    fn right_order(left: &Self, right: &Self) -> Option<bool> {
        match (left, right) {
            (Packet::Num(l), Packet::Num(r)) => {
                if l == r {
                    None
                } else {
                    Some(l < r)
                }
            }
            (Packet::List(l), Packet::List(r)) => Packet::right_order_list(l, r),
            (l @ Packet::Num(_), Packet::List(r)) => {
                Packet::right_order_list(core::slice::from_ref(l), r)
            }
            (Packet::List(l), r @ Packet::Num(_)) => {
                Packet::right_order_list(l, core::slice::from_ref(r))
            }
        }
    }

    fn right_order_list(left: &[Self], right: &[Self]) -> Option<bool> {
        let mut li = left.iter();
        let mut ri = right.iter();

        loop {
            match (li.next(), ri.next()) {
                (Some(l), Some(r)) => {
                    if let Some(order) = Packet::right_order(l, r) {
                        return Some(order);
                    }
                }
                (Some(_), None) => return Some(false),
                (None, Some(_)) => return Some(true),
                (None, None) => return None,
            }
        }
    }
}

fn parse_pair<A: Allocator + Copy>(
    alloc: A,
    input: &str,
) -> nom::IResult<&str, (Packet<A>, Packet<A>), ()> {
    terminated(
        separated_pair(
            |i| Packet::parse(alloc, i),
            line_ending,
            |i| Packet::parse(alloc, i),
        ),
        line_ending,
    )(input)
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;
    assert_eq!(day13(&bump, example).0, 13);
    assert_eq!(day13(&bump, example).1, 140);
}
