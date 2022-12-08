use alloc::vec::Vec;
use core::alloc::Allocator;
use core::fmt::Debug;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day08<A: Allocator + Debug>(alloc: A, input: &str) -> (u32, u32) {
    // Assume input is a square
    let dim = input.find('\n').unwrap() as u32;

    let input = input.as_bytes();

    macro_rules! get_tree_height {
        ($x:expr, $y:expr) => {
            (input[($y * (dim + 1) + $x) as usize] - b'0') as i8
        };
    }

    let mut visible = Vec::new_in(&alloc);
    visible.resize((dim * dim) as usize, false);

    macro_rules! set_visible {
        ($x:expr, $y:expr) => {
            visible[($y * dim + $x) as usize] = true;
        };
    }

    let mut scores = Vec::new_in(&alloc);
    scores.resize((dim * dim) as usize, 1u32);

    macro_rules! insert_score {
        ($x:expr, $y:expr, $score:expr) => {
            scores[($y * dim + $x) as usize] *= $score;
        };
    }

    for y in 0..dim {
        let mut tallest_up = -1;
        let mut tallest_down = -1;
        let mut tallest_left = -1;
        let mut tallest_right = -1;
        for x in 0..dim {
            {
                // LEFT
                let h = get_tree_height!(x, y);

                if h > tallest_left {
                    set_visible!(x, y);
                    tallest_left = h;
                }

                let mut score = 0;
                while x > score {
                    score += 1;
                    let next = x - score;

                    if get_tree_height!(next, y) >= h {
                        break;
                    }
                }

                insert_score!(x, y, score);
            }

            {
                // RIGHT
                let x = dim - 1 - x;
                let h = get_tree_height!(x, y);

                if h > tallest_right {
                    set_visible!(x, y);
                    tallest_right = h;
                }

                let mut score = 0;
                while x < dim - 1 - score {
                    score += 1;
                    let next = x + score;

                    if get_tree_height!(next, y) >= h {
                        break;
                    }
                }

                insert_score!(x, y, score);
            }

            {
                // UP
                let (x, y) = (y, x);
                let h = get_tree_height!(x, y);

                if h > tallest_up {
                    set_visible!(x, y);
                    tallest_up = h;
                }

                let mut score = 0;
                while y > score {
                    score += 1;
                    let next = y - score;

                    if get_tree_height!(x, next) >= h {
                        break;
                    }
                }

                insert_score!(x, y, score);
            }

            {
                // DOWN
                let (x, y) = (y, x);
                let y = dim - 1 - y;
                let h = get_tree_height!(x, y);

                if h > tallest_down {
                    set_visible!(x, y);
                    tallest_down = h;
                }

                let mut score = 0;
                while y < dim - 1 - score {
                    score += 1;
                    let next = y + score;

                    if get_tree_height!(x, next) >= h {
                        break;
                    }
                }

                insert_score!(x, y, score);
            }
        }
    }

    let part1 = visible.iter().filter(|v| **v).count() as u32;
    let part2 = scores.into_iter().max().unwrap();

    (part1, part2)
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"30373
25512
65332
33549
35390
"#;
    assert_eq!(day08(&bump, example).0, 21);
    assert_eq!(day08(&bump, example).1, 8);
}
