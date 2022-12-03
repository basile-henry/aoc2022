use alloc::boxed::Box;
use alloc::vec::Vec;
use core::alloc::Allocator;
use core::fmt::Debug;

use crate::hash_set;

use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day03<A: Allocator + Debug>(alloc: A, input: &[u8]) -> (usize, usize) {
    let (part1, (part2, _)) = fold_many0(
        terminated(|i| Rucksack::parse(&alloc, i), line_ending),
        || (0, (0, Vec::new_in(&alloc))),
        |(mut part1, (mut part2, mut window)), rucksack| {
            part1 += priority(rucksack.item_in_both(&alloc).unwrap());

            window.push(rucksack);

            if window.len() == 3 {
                part2 += priority(badge(&alloc, window.as_slice()).unwrap());
                window.clear();
            }

            (part1, (part2, window))
        },
    )(input)
    .unwrap()
    .1;

    (part1, part2)
}

struct Rucksack<A: Allocator> {
    comp_1: Box<[u8], A>,
    comp_2: Box<[u8], A>,
}

impl<A: Allocator + Copy> Rucksack<A> {
    fn parse(alloc: A, input: &[u8]) -> nom::IResult<&[u8], Rucksack<A>, ()> {
        map(alpha0, |items: &[u8]| {
            let (comp_1, comp_2) = items.split_at(items.len() / 2);
            let comp_1 = comp_1.to_vec_in(alloc).into_boxed_slice();
            let comp_2 = comp_2.to_vec_in(alloc).into_boxed_slice();
            Rucksack { comp_1, comp_2 }
        })(input)
    }

    fn item_in_both(&self, alloc: A) -> Option<u8> {
        let seen_in_comp_1 = hash_set!(alloc, self.comp_1.iter());
        let seen_in_comp_2 = hash_set!(alloc, self.comp_2.iter());

        seen_in_comp_1.intersection(&seen_in_comp_2).next().copied()
    }
}

fn priority(c: u8) -> usize {
    if (b'a'..=b'z').contains(&c) {
        (c - b'a' + 1).into()
    } else if (b'A'..=b'Z').contains(&c) {
        (c - b'A' + 27).into()
    } else {
        let c: char = c.into();
        panic!("item out of range \"{c}\"")
    }
}

fn badge<A: Allocator, B: Allocator>(alloc: A, elves: &[Rucksack<B>]) -> Option<u8> {
    let mut items = elves
        .iter()
        .map(|r| hash_set!(&alloc, r.comp_1.iter().chain(r.comp_2.iter())));

    let mut tmp = hash_set!(&alloc);

    let mut seen = items.next().unwrap();

    for set in items {
        tmp.clear();
        seen.intersection(&set).copied().collect_into(&mut tmp);

        core::mem::swap(&mut seen, &mut tmp);
    }

    seen.into_iter().next()
}

#[test]
fn both_paths() {
    let bump = bumpalo::Bump::new();
    let example = br#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
    assert_eq!(day03(&bump, example).0, 157);
    assert_eq!(day03(&bump, example).1, 70);
}
