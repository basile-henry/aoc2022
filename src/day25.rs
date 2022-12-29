use bumpalo::collections::String;
use bumpalo::Bump;

use nom::branch::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
pub fn day25<'bump>(bump: &'bump Bump, input: &str) -> (&'bump str, usize) {
    let sum = fold_many0(terminated(snafu_parse, line_ending), || 0, |sum, x| sum + x)(input)
        .unwrap()
        .1;

    let part1 = snafu_from(bump, sum);

    (part1, 0)
}

fn snafu_parse(input: &str) -> nom::IResult<&str, isize, ()> {
    fold_many1(
        alt((
            map(char('2'), |_| 2),
            map(char('1'), |_| 1),
            map(char('0'), |_| 0),
            map(char('-'), |_| -1),
            map(char('='), |_| -2),
        )),
        || 0,
        |n, d| n * 5 + d,
    )(input)
}

fn snafu_from(bump: &Bump, mut input: isize) -> &str {
    let mut chars = Vec::new_in(bump);

    while input != 0 {
        let d = input % 5;
        input /= 5;

        match d {
            0 => chars.push("0"),
            1 => chars.push("1"),
            2 => chars.push("2"),
            3 => {
                chars.push("=");
                input += 1;
            }
            4 => {
                chars.push("-");
                input += 1;
            }
            _ => panic!("Unexpected"),
        }
    }

    let mut out = String::new_in(bump);
    chars.into_iter().rev().collect_into(&mut out);
    out.into_bump_str()
}

#[test]
fn snafu_parse_test() {
    assert_eq!(snafu_parse("1"), Ok(("", 1)));
    assert_eq!(snafu_parse("2"), Ok(("", 2)));
    assert_eq!(snafu_parse("1="), Ok(("", 3)));
    assert_eq!(snafu_parse("1-"), Ok(("", 4)));
    assert_eq!(snafu_parse("10"), Ok(("", 5)));
    assert_eq!(snafu_parse("11"), Ok(("", 6)));
    assert_eq!(snafu_parse("12"), Ok(("", 7)));
    assert_eq!(snafu_parse("2="), Ok(("", 8)));
    assert_eq!(snafu_parse("2-"), Ok(("", 9)));
    assert_eq!(snafu_parse("20"), Ok(("", 10)));
    assert_eq!(snafu_parse("1=0"), Ok(("", 15)));
    assert_eq!(snafu_parse("1-0"), Ok(("", 20)));
    assert_eq!(snafu_parse("1=11-2"), Ok(("", 2022)));
    assert_eq!(snafu_parse("1-0---0"), Ok(("", 12345)));
    assert_eq!(snafu_parse("1121-1110-1=0"), Ok(("", 314159265)));
}

#[test]
fn snafu_from_test() {
    let bump = bumpalo::Bump::new();

    assert_eq!(snafu_from(&bump, 1), "1");
    assert_eq!(snafu_from(&bump, 2), "2");
    assert_eq!(snafu_from(&bump, 3), "1=");
    assert_eq!(snafu_from(&bump, 4), "1-");
    assert_eq!(snafu_from(&bump, 5), "10");
    assert_eq!(snafu_from(&bump, 6), "11");
    assert_eq!(snafu_from(&bump, 7), "12");
    assert_eq!(snafu_from(&bump, 8), "2=");
    assert_eq!(snafu_from(&bump, 9), "2-");
    assert_eq!(snafu_from(&bump, 10), "20");
    assert_eq!(snafu_from(&bump, 15), "1=0");
    assert_eq!(snafu_from(&bump, 20), "1-0");
    assert_eq!(snafu_from(&bump, 2022), "1=11-2");
    assert_eq!(snafu_from(&bump, 12345), "1-0---0");
    assert_eq!(snafu_from(&bump, 314159265), "1121-1110-1=0");
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"#;
    assert_eq!(day25(&bump, example).0, "2=-1=0");
    assert_eq!(day25(&bump, example).1, 0);
}
