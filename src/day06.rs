use crate::bitset::U32Set;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day06(input: &[u8]) -> (usize, usize) {
    let solve = |window_size| {
        let (offset, _) = input
            .windows(window_size)
            .enumerate()
            .map(|(i, w)| (i, U32Set::from_iter(w.iter().map(|c| c - b'a'))))
            .find(|(_, s)| s.count() == window_size)
            .unwrap();
        offset + window_size
    };

    (solve(4), solve(14))
}

#[test]
fn both_parts() {
    assert_eq!(day06(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb").0, 7);
    assert_eq!(day06(b"bvwbjplbgvbhsrlpgdmjqwftvncz").0, 5);
    assert_eq!(day06(b"nppdvjthqldpwncqszvftbrmjlhg").0, 6);
    assert_eq!(day06(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").0, 10);
    assert_eq!(day06(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").0, 11);

    assert_eq!(day06(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb").1, 19);
    assert_eq!(day06(b"bvwbjplbgvbhsrlpgdmjqwftvncz").1, 23);
    assert_eq!(day06(b"nppdvjthqldpwncqszvftbrmjlhg").1, 23);
    assert_eq!(day06(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").1, 29);
    assert_eq!(day06(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").1, 26);
}
