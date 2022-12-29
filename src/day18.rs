use alloc::vec::Vec;
use core::alloc::Allocator;
use core::fmt::Debug;

use crate::hash::HashSet;
use crate::hash_set;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
pub fn day18<A: Allocator + Debug + Copy>(alloc: A, input: &str) -> (usize, usize) {
    let (bounds @ (a_min, a_max, b_min, b_max, c_min, c_max), cubes) = fold_many0(
        map(
            tuple((u8::<&str, ()>, char(','), u8, char(','), u8, newline)),
            |(a, _, b, _, c, _)| (a, b, c),
        ),
        || {
            (
                (u8::MAX, u8::MIN, u8::MAX, u8::MIN, u8::MAX, u8::MIN),
                hash_set!(3000, alloc),
            )
        },
        |((a_min, a_max, b_min, b_max, c_min, c_max), mut hm), (a, b, c)| {
            hm.insert((a, b, c));
            (
                (
                    a_min.min(a),
                    a_max.max(a),
                    b_min.min(b),
                    b_max.max(b),
                    c_min.min(c),
                    c_max.max(c),
                ),
                hm,
            )
        },
    )(input)
    .unwrap()
    .1;

    let as_ = a_min..=a_max;
    let bs = b_min..=b_max;
    let cs = c_min..=c_max;

    let mut bound_points = gen_points(&as_, &bs, |a, b| (a, b, c_min))
        .chain(gen_points(&as_, &bs, |a, b| (a, b, c_max)))
        .chain(gen_points(&bs, &cs, |b, c| (a_min, b, c)))
        .chain(gen_points(&bs, &cs, |b, c| (a_max, b, c)))
        .chain(gen_points(&as_, &cs, |a, c| (a, b_min, c)))
        .chain(gen_points(&as_, &cs, |a, c| (a, b_max, c)));

    let mut reachable_from_outside: HashSet<(u8, u8, u8), A> = hash_set!(alloc);

    let mut to_visit: Vec<(u8, u8, u8), A> = Vec::new_in(alloc);

    while let Some(current) = to_visit.pop().or_else(|| bound_points.next()) {
        if cubes.contains(&current) {
            continue;
        }

        reachable_from_outside.insert(current);

        to_visit.extend(neighbours(&current).flatten().filter(|neighbour| {
            // Already seen
            if reachable_from_outside.contains(neighbour) {
                return false;
            }

            // Outside the bounds
            if outside_bounds(neighbour, &bounds) {
                return false;
            }

            true
        }));
    }

    cubes.iter().flat_map(neighbours).fold(
        (0, 0),
        |(mut open_faces, mut face_reachable), neighbour| {
            if let Some(neighbour) = neighbour {
                if !cubes.contains(&neighbour) {
                    open_faces += 1;

                    if reachable_from_outside.contains(&neighbour)
                        || outside_bounds(&neighbour, &bounds)
                    {
                        face_reachable += 1;
                    }
                }
            } else {
                open_faces += 1;
                face_reachable += 1;
            }

            (open_faces, face_reachable)
        },
    )
}

fn gen_points<'a>(
    xs: &'a (impl Iterator<Item = u8> + Clone),
    ys: &'a (impl Iterator<Item = u8> + Clone),
    f: impl Fn(u8, u8) -> (u8, u8, u8) + Copy + 'a,
) -> impl Iterator<Item = (u8, u8, u8)> + 'a {
    xs.clone()
        .flat_map(move |x| ys.clone().map(move |y| f(x, y)))
}

fn outside_bounds(
    (a, b, c): &(u8, u8, u8),
    (a_min, a_max, b_min, b_max, c_min, c_max): &(u8, u8, u8, u8, u8, u8),
) -> bool {
    a < a_min || a > a_max || b < b_min || b > b_max || c < c_min || c > c_max
}

fn neighbours(&(a, b, c): &(u8, u8, u8)) -> impl Iterator<Item = Option<(u8, u8, u8)>> {
    fn neighbour((a, b, c): (u8, u8, u8), (da, db, dc): (i8, i8, i8)) -> Option<(u8, u8, u8)> {
        let a = a.checked_add_signed(da)?;
        let b = b.checked_add_signed(db)?;
        let c = c.checked_add_signed(dc)?;
        Some((a, b, c))
    }

    [
        neighbour((a, b, c), (-1, 0, 0)),
        neighbour((a, b, c), (1, 0, 0)),
        neighbour((a, b, c), (0, -1, 0)),
        neighbour((a, b, c), (0, 1, 0)),
        neighbour((a, b, c), (0, 0, -1)),
        neighbour((a, b, c), (0, 0, 1)),
    ]
    .into_iter()
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;
    assert_eq!(day18(&bump, example).0, 64);
    assert_eq!(day18(&bump, example).1, 58);
}
