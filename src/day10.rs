use core::iter::once;
use core::ops::RangeInclusive;

use bumpalo::Bump;

use bumpalo::collections::String;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day10<'bump>(bump: &'bump Bump, input: &str) -> (usize, &'bump str) {
    let (_cpu, part1, crt) = fold_many0(
        terminated(Instruction::parse, newline),
        || (Cpu::new(), 0, Crt::new()),
        |(mut cpu, mut signal_strength, mut crt), instr| {
            for state in cpu.steps(instr) {
                let x = state.x;

                if state.cycle_count >= 20 && (state.cycle_count - 20) % 40 == 0 {
                    signal_strength += state.cycle_count * x as usize;
                }

                let sprite = (x - 1)..=(x + 1);

                crt.set(state.cycle_count, sprite);
            }

            (cpu, signal_strength, crt)
        },
    )(input)
    .unwrap()
    .1;

    (part1, crt.render(bump))
}

enum Instruction {
    AddX(i32),
    NoOp,
}

impl Instruction {
    fn parse(input: &str) -> nom::IResult<&str, Instruction, ()> {
        alt((
            map(preceded(tag("addx "), i32), Instruction::AddX),
            map(tag("noop"), |_| Instruction::NoOp),
        ))(input)
    }
}

#[derive(Clone, Copy)]
struct Cpu {
    x: i32,
    cycle_count: usize,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            x: 1,
            cycle_count: 1,
        }
    }

    fn steps(&mut self, instruction: Instruction) -> impl Iterator<Item = Cpu> {
        match instruction {
            Instruction::AddX(y) => {
                let next = {
                    let mut cpu = *self;
                    cpu.cycle_count += 2;
                    cpu.x += y;
                    cpu
                };

                let states = once({
                    let mut cpu = *self;
                    cpu.cycle_count += 1;
                    cpu
                })
                .chain(Some(next));

                *self = next;

                states
            }
            Instruction::NoOp => {
                let next = {
                    let mut cpu = *self;
                    cpu.cycle_count += 1;
                    cpu
                };

                let states = once(next).chain(None);

                *self = next;

                states
            }
        }
    }
}

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;
const CRT_SIZE: usize = CRT_WIDTH * CRT_HEIGHT;

struct Crt {
    screen: [bool; CRT_SIZE],
}

impl Crt {
    fn new() -> Self {
        Crt {
            screen: [false; CRT_SIZE],
        }
    }

    fn set(&mut self, cycle_count: usize, sprite: RangeInclusive<i32>) {
        if cycle_count <= CRT_SIZE {
            let pos = cycle_count - 1;

            self.screen[pos] = sprite.contains(&((pos % CRT_WIDTH) as i32));
        }
    }

    fn render<'bump>(&self, bump: &'bump Bump) -> &'bump str {
        let mut out = String::with_capacity_in(CRT_SIZE + CRT_HEIGHT, bump);

        for y in 0..CRT_HEIGHT {
            out.push('\n');

            for x in 0..CRT_WIDTH {
                out.push(if self.screen[y * CRT_WIDTH + x] {
                    '#'
                } else {
                    '.'
                });
            }
        }

        out.into_bump_str()
    }
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;
    assert_eq!(day10(&bump, example).0, 13140);
}
