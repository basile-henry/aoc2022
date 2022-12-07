use nom::branch::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day02(input: &str) -> (u32, u32) {
    fold_many0(
        terminated(parse, line_ending),
        || (0, 0),
        |(part1, part2), (round1, round2)| {
            let score1 = score(round1.me, round1.outcome());
            let score2 = score(round2.me(), round2.outcome);
            (part1 + score1, part2 + score2)
        },
    )(input.as_bytes())
    .unwrap()
    .1
}

fn parse(input: &[u8]) -> nom::IResult<&[u8], (Round1, Round2), ()> {
    map(
        separated_pair(
            alt((
                map(char('A'), |_| Play::Rock),
                map(char('B'), |_| Play::Paper),
                map(char('C'), |_| Play::Scissors),
            )),
            char(' '),
            alt((
                map(char('X'), |_| (Play::Rock, Outcome::Lose)),
                map(char('Y'), |_| (Play::Paper, Outcome::Draw)),
                map(char('Z'), |_| (Play::Scissors, Outcome::Win)),
            )),
        ),
        |(opponent, (me, outcome))| (Round1 { opponent, me }, Round2 { opponent, outcome }),
    )(input)
}

fn score(me: Play, outcome: Outcome) -> u32 {
    1 + me as u32 + 3 * outcome as u32
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
fn both_parts() {
    let example = r#"A Y
B X
C Z
"#;
    assert_eq!(day02(example).0, 15);
    assert_eq!(day02(example).1, 12);
}
