use alloc::str::from_utf8;
use alloc::vec::Vec;
use core::alloc::Allocator;
use core::fmt::Debug;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day01<A: Allocator + Debug>(alloc: A, input: &[u8]) -> (i32, i32) {
    let input = from_utf8(input).unwrap();

    let most_calories = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| str::parse::<i32>(l).unwrap()).sum())
        .fold(Vec::new_in(alloc), |mut top, calories| {
            let ix = top.partition_point(|x| x > &calories);
            top.insert(ix, calories);
            top.truncate(3);
            top
        });

    (most_calories[0], most_calories.into_iter().sum())
}

#[test]
fn both_paths() {
    let bump = bumpalo::Bump::new();
    let example = br#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;
    assert_eq!(day01(&bump, example).0, 24000);
    assert_eq!(day01(&bump, example).1, 45000);
}
