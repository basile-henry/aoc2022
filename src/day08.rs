use alloc::vec::Vec;
use core::alloc::Allocator;
use core::fmt::Debug;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day08<A: Allocator + Debug>(alloc: A, input: &str) -> (usize, usize) {
    // Assume input is a square
    let dim = input.find('\n').unwrap();

    let input = input.as_bytes();

    macro_rules! get_tree_height {
        ($x:expr, $y:expr) => {
            (input[($y * (dim + 1) + $x) as usize] - b'0') as i8
        };
    }

    let mut visible_count = 0;

    let mut visible = Vec::new_in(&alloc);
    visible.resize(dim * dim, false);

    macro_rules! set_visible {
        ($x:expr, $y:expr) => {
            if let Some(vis) = visible.get_mut($y * dim + $x) {
                if !*vis {
                    *vis = true;
                    visible_count += 1;
                }
            }
        };
    }

    let mut max_score = 0;

    for y in 0..dim {
        let mut tallest_up = -1;
        let mut tallest_down = -1;
        let mut tallest_left = -1;
        let mut tallest_right = -1;
        for x in 0..dim {
            // Visibility
            {
                // Left - traverse from left edge
                let h = get_tree_height!(x, y);

                if h > tallest_left {
                    set_visible!(x, y);
                    tallest_left = h;
                }
            }

            {
                // Right - traverse from right edge
                let x = dim - 1 - x;
                let h = get_tree_height!(x, y);

                if h > tallest_right {
                    set_visible!(x, y);
                    tallest_right = h;
                }
            }

            {
                // Up - traverse from top edge
                let (x, y) = (y, x);
                let h = get_tree_height!(x, y);

                if h > tallest_up {
                    set_visible!(x, y);
                    tallest_up = h;
                }
            }

            {
                // Down - traverse from bottom edge
                let (x, y) = (y, x);
                let y = dim - 1 - y;
                let h = get_tree_height!(x, y);

                if h > tallest_down {
                    set_visible!(x, y);
                    tallest_down = h;
                }
            }

            // Score
            let h = get_tree_height!(x, y);
            let mut tree_score = 1;

            {
                // Left
                let mut score = 0;
                while x > score {
                    score += 1;
                    let next = x - score;

                    if get_tree_height!(next, y) >= h {
                        break;
                    }
                }

                tree_score *= score;
            }

            {
                // Right
                let mut score = 0;
                while x < dim - 1 - score {
                    score += 1;
                    let next = x + score;

                    if get_tree_height!(next, y) >= h {
                        break;
                    }
                }

                tree_score *= score;
            }

            {
                // Up
                let mut score = 0;
                while y > score {
                    score += 1;
                    let next = y - score;

                    if get_tree_height!(x, next) >= h {
                        break;
                    }
                }

                tree_score *= score;
            }
            {
                // Down
                let mut score = 0;
                while y < dim - 1 - score {
                    score += 1;
                    let next = y + score;

                    if get_tree_height!(x, next) >= h {
                        break;
                    }
                }

                tree_score *= score;
            }

            max_score = max_score.max(tree_score);
        }
    }

    let part1 = visible_count;
    let part2 = max_score;

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
