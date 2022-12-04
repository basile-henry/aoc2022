use core::ops::RangeInclusive;

use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day04(input: &[u8]) -> (u32, u32) {
    fold_many0(
        terminated(parse_elf_pair, line_ending),
        || (0, 0),
        |(part1, part2), (elf1, elf2)| {
            (
                part1 + if full_overlap(&elf1, &elf2) { 1 } else { 0 },
                part2 + if partial_overlap(&elf1, &elf2) { 1 } else { 0 },
            )
        },
    )(input)
    .unwrap()
    .1
}

fn partial_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    let partial_a_in_b = b.contains(a.start()) || b.contains(a.end());
    let partial_b_in_a = a.contains(b.start()) || a.contains(b.end());

    partial_a_in_b || partial_b_in_a
}

fn full_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    let a_in_b = b.contains(a.start()) && b.contains(a.end());
    let b_in_a = a.contains(b.start()) && a.contains(b.end());

    a_in_b || b_in_a
}

fn parse_range(input: &[u8]) -> nom::IResult<&[u8], RangeInclusive<u32>, ()> {
    map(separated_pair(u32, char('-'), u32), |(from, to)| from..=to)(input)
}

fn parse_elf_pair(
    input: &[u8],
) -> nom::IResult<&[u8], (RangeInclusive<u32>, RangeInclusive<u32>), ()> {
    separated_pair(parse_range, char(','), parse_range)(input)
}

#[test]
fn both_paths() {
    let example = br#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
    assert_eq!(day04(example).0, 2);
    assert_eq!(day04(example).1, 4);
}
