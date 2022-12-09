use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

fn is_unique<T>(a: T, b: T, c: T, d: T) -> bool
where
    T: Eq + Hash,
{
    let s = HashSet::from([a, b, c, d]);
    s.len() == 4
}

fn start_index(input: &str) -> Option<usize> {
    input
        .chars()
        .tuple_windows()
        .position(|(a, b, c, d)| is_unique(a, b, c, d))
        .map(|pos| pos + 4)
}

fn start_message(input: &str) -> Option<usize> {
    const SEQ_SIZE: usize = 14;
    input
        .as_bytes()
        .windows(SEQ_SIZE)
        .position(|w| w.iter().unique().count() == SEQ_SIZE)
        .map(|pos| pos + SEQ_SIZE)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Index: {:?}", start_index(input));
    println!("Message: {:?}", start_message(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_things_work() {
        assert_eq!(Some(5), start_index("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), start_index("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(10), start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    }

    #[test]
    fn test_message() {
        assert_eq!(Some(23), start_message("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(23), start_message("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(29), start_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    }
}
