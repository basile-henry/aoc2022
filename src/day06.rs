use crate::bitset::U32Set;

#[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
pub fn day06(input: &str) -> (usize, usize) {
    let solve = |window_size| {
        let (offset, _) = input
            .as_bytes()
            .windows(window_size)
            .enumerate()
            .find(|(_, w)| {
                U32Set::insert_only_new(&mut U32Set::empty(), w.iter().map(|c| c - b'a'))
            })
            .unwrap();
        offset + window_size
    };

    (solve(4), solve(14))
}

#[test]
fn both_parts() {
    assert_eq!(day06("mjqjpqmgbljsphdztnvjfqwrcgsmlb").0, 7);
    assert_eq!(day06("bvwbjplbgvbhsrlpgdmjqwftvncz").0, 5);
    assert_eq!(day06("nppdvjthqldpwncqszvftbrmjlhg").0, 6);
    assert_eq!(day06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").0, 10);
    assert_eq!(day06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").0, 11);

    assert_eq!(day06("mjqjpqmgbljsphdztnvjfqwrcgsmlb").1, 19);
    assert_eq!(day06("bvwbjplbgvbhsrlpgdmjqwftvncz").1, 23);
    assert_eq!(day06("nppdvjthqldpwncqszvftbrmjlhg").1, 23);
    assert_eq!(day06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").1, 29);
    assert_eq!(day06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").1, 26);
}
