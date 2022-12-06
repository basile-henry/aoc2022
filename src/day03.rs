use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

use crate::bitset::U64Set;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day03(input: &[u8]) -> (u32, u32) {
    let (part1, (part2, _)) = fold_many0(
        terminated(Rucksack::parse, line_ending),
        || (0, (0, heapless::Vec::<_, 3>::new())),
        |(mut part1, (mut part2, mut window)), rucksack| {
            part1 += rucksack.item_in_both_priority().unwrap() as u32;

            window.push(rucksack).unwrap();

            if window.len() == 3 {
                part2 += badge(window.as_slice()).unwrap() as u32;
                window.clear();
            }

            (part1, (part2, window))
        },
    )(input)
    .unwrap()
    .1;

    (part1, part2)
}

#[derive(Debug)]
struct Rucksack<'a> {
    comp_1: &'a [u8],
    comp_2: &'a [u8],
}

impl<'a> Rucksack<'a> {
    fn parse(input: &'a [u8]) -> nom::IResult<&'a [u8], Rucksack<'a>, ()> {
        map(alpha0, |items: &[u8]| {
            let (comp_1, comp_2) = items.split_at(items.len() / 2);
            Rucksack { comp_1, comp_2 }
        })(input)
    }

    fn item_in_both_priority(&self) -> Option<u8> {
        let seen_in_comp_1 = U64Set::from_iter(self.comp_1.iter().copied().map(priority));
        let seen_in_comp_2 = U64Set::from_iter(self.comp_2.iter().copied().map(priority));

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

fn badge(elves: &[Rucksack<'_>]) -> Option<u8> {
    let mut items = elves.iter().map(|r| {
        U64Set::from_iter(
            r.comp_1
                .iter()
                .chain(r.comp_2.iter())
                .copied()
                .map(priority),
        )
    });

    let mut seen = items.next().unwrap();

    for other in items {
        seen = seen.intersection(&other)
    }

    let badge = seen.iter().next();

    badge
}

#[test]
fn both_paths() {
    let example = br#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
    assert_eq!(day03(example).0, 157);
    assert_eq!(day03(example).1, 70);
}
