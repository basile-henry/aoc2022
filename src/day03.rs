use alloc::boxed::Box;
use alloc::vec::Vec;
use core::alloc::Allocator;
use core::fmt::Debug;

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
            part1 += rucksack.item_in_both().unwrap() as usize;

            window.push(rucksack);

            if window.len() == 3 {
                part2 += badge(window.as_slice()).unwrap() as usize;
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

            let mut v1 = Vec::new_in(alloc);
            v1.extend(comp_1.iter().copied().map(priority));
            let comp_1 = v1.into_boxed_slice();

            let mut v2 = Vec::new_in(alloc);
            v2.extend(comp_2.iter().copied().map(priority));
            let comp_2 = v2.into_boxed_slice();

            Rucksack { comp_1, comp_2 }
        })(input)
    }

    fn item_in_both(&self) -> Option<u8> {
        let seen_in_comp_1 = U64Set::from_iter(self.comp_1.iter().copied());
        let seen_in_comp_2 = U64Set::from_iter(self.comp_2.iter().copied());

        seen_in_comp_1.intersection(&seen_in_comp_2).iter().next()
    }
}

// Turn an item into its priority
fn priority(c: u8) -> u8 {
    if (b'a'..=b'z').contains(&c) {
        c - b'a' + 1
    } else if (b'A'..=b'Z').contains(&c) {
        c - b'A' + 27
    } else {
        let c: char = c.into();
        panic!("item out of range \"{c}\"")
    }
}

fn badge<A: Allocator>(elves: &[Rucksack<A>]) -> Option<u8> {
    let mut items = elves
        .iter()
        .map(|r| U64Set::from_iter(r.comp_1.iter().chain(r.comp_2.iter()).copied()));

    let mut seen = items.next().unwrap();

    for other in items {
        seen = seen.intersection(&other)
    }

    let badge = seen.iter().next();

    badge
}

struct U64Set(u64);

impl U64Set {
    fn empty() -> Self {
        U64Set(0)
    }

    fn contains(&self, item: u8) -> bool {
        debug_assert!(item < 64);
        (self.0 & 1 << item) > 0
    }

    fn insert(&mut self, item: u8) {
        debug_assert!(item < 64);
        self.0 |= 1 << item;
    }

    fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    fn iter(&'_ self) -> impl Iterator<Item = u8> + '_ {
        (0..64).filter(|x| self.contains(*x))
    }
}

impl FromIterator<u8> for U64Set {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut set = Self::empty();

        for x in iter {
            set.insert(x);
        }

        set
    }
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
