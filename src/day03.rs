use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

use crate::bitset::U64Set;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day03(input: &str) -> (u32, u32) {
    fold_many0(
        terminated(Rucksack::parse, line_ending),
        || ((0, 0), heapless::Vec::<_, 3>::new()),
        |((mut part1, mut part2), mut window), rucksack| {
            part1 += rucksack.item_in_both_priority().unwrap() as u32;

            window.push(rucksack).unwrap();

            if window.len() == 3 {
                part2 += badge(window.as_slice()).unwrap() as u32;
                window.clear();
            }

            ((part1, part2), window)
        },
    )(input.as_bytes())
    .unwrap()
    .1
     .0
}

#[derive(Debug)]
struct Rucksack {
    comp_1: U64Set,
    comp_2: U64Set,
}

impl Rucksack {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Rucksack, ()> {
        map(alpha0, |items: &[u8]| {
            let (comp_1, comp_2) = items.split_at(items.len() / 2);
            let comp_1 = U64Set::from_iter(comp_1.iter().copied().map(priority));
            let comp_2 = U64Set::from_iter(comp_2.iter().copied().map(priority));
            Rucksack { comp_1, comp_2 }
        })(input)
    }

    fn item_in_both_priority(&self) -> Option<u8> {
        self.comp_1.intersection(&self.comp_2).iter().next()
    }
}

// Turn an item into its priority
fn priority(c: u8) -> u8 {
    if c <= b'Z' {
        c - b'A' + 27
    } else {
        c - b'a' + 1
    }
}

fn badge(elves: &[Rucksack]) -> Option<u8> {
    elves
        .iter()
        .map(|r| r.comp_1.union(&r.comp_2))
        .reduce(|a, b| a.intersection(&b))
        .unwrap()
        .iter()
        .next()
}

#[test]
fn both_parts() {
    let example = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
    assert_eq!(day03(example).0, 157);
    assert_eq!(day03(example).1, 70);
}
