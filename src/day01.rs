#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day01(input: &str) -> (u32, u32) {
    let most_calories = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| str::parse::<u32>(l).unwrap()).sum())
        .fold(heapless::Vec::<u32, 4>::new(), |mut top, calories| {
            let ix = top.partition_point(|x| x > &calories);
            top.insert(ix, calories).unwrap();
            top.truncate(3);
            top
        });

    (most_calories[0], most_calories.into_iter().sum())
}

#[test]
fn both_parts() {
    let example = r#"1000
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
    assert_eq!(day01(example).0, 24000);
    assert_eq!(day01(example).1, 45000);
}
