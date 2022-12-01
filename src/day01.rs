use alloc::str::from_utf8;
use alloc::vec::Vec;
use core::alloc::Allocator;
use core::cmp::Reverse;

pub fn solve(alloc: impl Allocator, input: &[u8]) -> (i32, i32) {
    let mut most_calories = Vec::new_in(alloc);

    let mut elf_calories = 0;
    for line in input.split(|c| *c == b'\n') {
        if line.is_empty() {
            most_calories.push(elf_calories);
            most_calories.sort_by_key(|&x| Reverse(x)); // To sort in reverse order
            most_calories.truncate(3);
            elf_calories = 0;
        } else {
            elf_calories += from_utf8(line).unwrap().parse::<i32>().unwrap();
        }
    }

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
    assert_eq!(solve(&bump, example).0, 24000);
    assert_eq!(solve(&bump, example).1, 45000);
}
