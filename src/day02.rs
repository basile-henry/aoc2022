use core::alloc::Allocator;
use core::fmt::Debug;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[tracing::instrument]
pub fn day02<A: Allocator + Debug>(alloc: A, input: &[u8]) -> (usize, usize) {
    fold_many0(
        terminated(parse, line_ending),
        || (0, 0),
        |(part1, part2), (round1, round2)| {
            let score1 = score(round1.me, round1.outcome());
            let score2 = score(round2.me(), round2.outcome);
            (part1 + score1, part2 + score2)
        },
    )(input)
    .unwrap()
    .1
}

fn parse(input: &[u8]) -> nom::IResult<&[u8], (Round1, Round2), ()> {
    map(
        separated_pair(
            alt((
                map(tag("A"), |_| Play::Rock),
                map(tag("B"), |_| Play::Paper),
                map(tag("C"), |_| Play::Scissors),
            )),
            char(' '),
            alt((
                map(tag("X"), |_| (Play::Rock, Outcome::Lose)),
                map(tag("Y"), |_| (Play::Paper, Outcome::Draw)),
                map(tag("Z"), |_| (Play::Scissors, Outcome::Win)),
            )),
        ),
        |(opponent, (me, outcome))| (Round1 { opponent, me }, Round2 { opponent, outcome }),
    )(input)
}

fn score(me: Play, outcome: Outcome) -> usize {
    1 + me as usize + 3 * outcome as usize
}

#[derive(PartialEq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

struct Round1 {
    opponent: Play,
    me: Play,
}

impl Round1 {
    fn outcome(&self) -> Outcome {
        if self.me == self.opponent {
            Outcome::Draw
        } else {
            match (self.me, self.opponent) {
                (Play::Rock, Play::Scissors)
                | (Play::Paper, Play::Rock)
                | (Play::Scissors, Play::Paper) => Outcome::Win,
                _ => Outcome::Lose,
            }
        }
    }
}

struct Round2 {
    opponent: Play,
    outcome: Outcome,
}

impl Round2 {
    fn me(&self) -> Play {
        match (&self.outcome, self.opponent) {
            (Outcome::Lose, Play::Rock) => Play::Scissors,
            (Outcome::Lose, Play::Paper) => Play::Rock,
            (Outcome::Lose, Play::Scissors) => Play::Paper,
            (Outcome::Draw, x) => x,
            (Outcome::Win, Play::Rock) => Play::Paper,
            (Outcome::Win, Play::Paper) => Play::Scissors,
            (Outcome::Win, Play::Scissors) => Play::Rock,
        }
    }
}

#[test]
fn both_paths() {
    let bump = bumpalo::Bump::new();
    let example = br#"A Y
B X
C Z
"#;
    assert_eq!(day02(&bump, example).0, 15);
    assert_eq!(day02(&bump, example).1, 12);
}
