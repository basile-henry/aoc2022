use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use core::alloc::Allocator;
use core::cmp::Reverse;
use core::fmt::Debug;

use nom::branch::*;
use nom::bytes::complete::tag;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day11<A: Allocator + Debug>(alloc: A, input: &str) -> (usize, usize) {
    let parse = tracing::trace_span!("parse");
    let parse = parse.enter();

    let mut monkeys = fold_many0(
        terminated(|i| Monkey::parse(&alloc, i), opt(line_ending)),
        || Vec::with_capacity_in(8, &alloc),
        |mut v, m| {
            v.push(m);
            v
        },
    )(input)
    .unwrap()
    .1;

    drop(parse);

    let part1 = {
        let part1 = tracing::trace_span!("part1");
        let _part1 = part1.enter();

        let mut monkeys_tmp = Vec::with_capacity_in(monkeys.len(), &alloc);
        monkeys.clone_into(&mut monkeys_tmp);
        let mut monkeys = monkeys_tmp;

        let mut count = Vec::with_capacity_in(monkeys.len(), &alloc);
        count.resize(monkeys.len(), 0);

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                while let Some((j, item)) = monkeys[i].item_handle() {
                    count[i] += 1;
                    monkeys[j].items.push(item);
                }
            }
        }

        count.sort_by_key(|x| Reverse(*x));
        count[0] * count[1]
    };

    let part2 = {
        let part2 = tracing::trace_span!("part2");
        let _part2 = part2.enter();

        let mut count = Vec::with_capacity_in(monkeys.len(), &alloc);
        count.resize(monkeys.len(), 0);

        let modulo = monkeys.iter().map(|m| m.test_num).product();

        for _ in 0..10_000 {
            for i in 0..monkeys.len() {
                while let Some((j, item)) = monkeys[i].item_handle_2(modulo) {
                    count[i] += 1;
                    monkeys[j].items.push(item);
                }
            }
        }

        count.sort_by_key(|x| Reverse(*x));
        count[0] * count[1]
    };

    (part1, part2)
}

#[derive(Debug, Clone)]
enum Operation {
    Times(Option<u64>),
    Plus(Option<u64>),
}

impl Operation {
    fn parse(input: &str) -> nom::IResult<&str, Self, ()> {
        map(
            preceded(
                tag("new = old "),
                separated_pair(
                    alt((char('*'), char('+'))),
                    char(' '),
                    alt((map(u64, Some), map(tag("old"), |_| None))),
                ),
            ),
            |(o, x)| match o {
                '*' => Operation::Times(x),
                '+' => Operation::Plus(x),
                _ => unreachable!(),
            },
        )(input)
    }

    fn apply(&self, x: u64) -> u64 {
        match self {
            Operation::Times(Some(y)) => x * y,
            Operation::Times(None) => x * x,
            Operation::Plus(Some(y)) => x + y,
            Operation::Plus(None) => x + x,
        }
    }

    fn apply_modulo(&self, x: u64, m: u64) -> u64 {
        match self {
            Operation::Times(Some(y)) => ((x % m) * (y % m)) % m,
            Operation::Times(None) => ((x % m) * (x % m)) % m,
            Operation::Plus(Some(y)) => ((x % m) + (y % m)) % m,
            Operation::Plus(None) => ((x % m) + (x % m)) % m,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey<A: Allocator> {
    items: Vec<u64, A>,
    operation: Operation,
    test_num: u64,
    test_true: u8,
    test_false: u8,
}

impl<A: Allocator + Copy> Monkey<A> {
    fn parse(alloc: A, input: &str) -> nom::IResult<&str, Self, ()> {
        let (input, _) = tuple((tag("Monkey "), digit1, tag(":"), line_ending))(input)?;

        let (input, items) = preceded(tag("  Starting items: "), |i| parse_items(alloc, i))(input)?;

        let (input, operation) = terminated(
            preceded(tag("  Operation: "), Operation::parse),
            line_ending,
        )(input)?;

        let (input, test_num) =
            terminated(preceded(tag("  Test: divisible by "), u64), line_ending)(input)?;

        let (input, test_true) = terminated(
            preceded(tag("    If true: throw to monkey "), u8),
            line_ending,
        )(input)?;

        let (input, test_false) = terminated(
            preceded(tag("    If false: throw to monkey "), u8),
            line_ending,
        )(input)?;

        Ok((
            input,
            Monkey {
                items,
                operation,
                test_num,
                test_true,
                test_false,
            },
        ))
    }

    fn item_handle(&mut self) -> Option<(usize, u64)> {
        let item = self.items.pop()?;
        let item = self.operation.apply(item);
        let item = item / 3;

        if item % self.test_num == 0 {
            Some((self.test_true as usize, item))
        } else {
            Some((self.test_false as usize, item))
        }
    }

    fn item_handle_2(&mut self, m: u64) -> Option<(usize, u64)> {
        let item = self.items.pop()?;
        let item = self.operation.apply_modulo(item, m);

        if item % self.test_num == 0 {
            Some((self.test_true as usize, item))
        } else {
            Some((self.test_false as usize, item))
        }
    }
}

fn parse_items<A: Allocator + Copy>(alloc: A, input: &str) -> nom::IResult<&str, Vec<u64, A>, ()> {
    fold_many0(
        terminated(u64, alt((tag(", "), line_ending))),
        || Vec::with_capacity_in(64, alloc),
        |mut v, i| {
            v.push(i);
            v
        },
    )(input)
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;
    assert_eq!(day11(&bump, example).0, 10605);
    assert_eq!(day11(&bump, example).1, 2713310158);
}
