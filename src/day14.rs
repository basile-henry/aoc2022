use core::fmt::Debug;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day14(input: &str) -> (usize, usize) {
    let mut reservoir = [[Cell::Air; 1000]; 200];
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for line in input.lines() {
        let mut prev: Option<(usize, usize)> = None;

        for point in line.split(" -> ") {
            let (x, y) = point.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            min_x = min_x.min(x);
            max_x = max_x.max(x);
            max_y = max_y.max(y);

            match prev {
                Some((px, py)) => {
                    if px == x {
                        let from = py.min(y);
                        let to = py.max(y);

                        #[allow(clippy::needless_range_loop)]
                        for y in from..=to {
                            reservoir[y][x] = Cell::Rock;
                        }
                    } else if py == y {
                        let from = px.min(x);
                        let to = px.max(x);

                        for x in from..=to {
                            reservoir[y][x] = Cell::Rock;
                        }
                    } else {
                        panic!("Unexpected");
                    }
                }
                None => {
                    reservoir[y][x] = Cell::Rock;
                }
            }

            prev = Some((x, y));
        }
    }

    let mut fallen_to_rest = 0;

    while let Some((x, y)) = sand_fall_to_rest(&reservoir[..=max_y]) {
        reservoir[y][x] = Cell::Sand;
        fallen_to_rest += 1;
    }

    let part1 = fallen_to_rest;

    for cell in &mut reservoir[max_y + 2] {
        *cell = Cell::Rock;
    }

    while let Some((x, y)) = sand_fall_to_rest(&reservoir[..=max_y + 2]) {
        reservoir[y][x] = Cell::Sand;
        fallen_to_rest += 1;
    }

    let part2 = fallen_to_rest;

    (part1, part2)
}

fn sand_fall_to_rest(reservoir: &[[Cell; 1000]]) -> Option<(usize, usize)> {
    let mut x = 500;
    let mut y = 0;

    // Stuck at start
    if reservoir[y][x] != Cell::Air {
        return None;
    }

    loop {
        // Exits at the bottom
        if y + 1 >= reservoir.len() {
            return None;
        }

        // Try to move down
        if reservoir[y + 1][x] == Cell::Air {
            y += 1;

        // Try to move down left
        } else if reservoir[y + 1][x - 1] == Cell::Air {
            y += 1;
            x -= 1;
        // Try to move down right
        } else if reservoir[y + 1][x + 1] == Cell::Air {
            y += 1;
            x += 1;

        // Comes to a rest
        } else {
            return Some((x, y));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

#[test]
fn both_parts() {
    let example = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;
    assert_eq!(day14(example).0, 24);
    assert_eq!(day14(example).1, 93);
}
