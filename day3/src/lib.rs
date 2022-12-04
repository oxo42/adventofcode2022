use std::fs;

mod parser;

use std::collections::HashMap;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn score(items: Vec<char>) -> usize {
    let alphabet_scores: HashMap<char, usize> = ALPHABET
        .chars()
        .enumerate()
        .map(|(pos, e)| (e, pos + 1))
        .collect();

    items
        .into_iter()
        .map(|c| alphabet_scores.get(&c).unwrap())
        .sum()
}

fn groups_score(contents: &str) -> eyre::Result<usize> {
    let alphabet_scores: HashMap<char, usize> = ALPHABET
        .chars()
        .enumerate()
        .map(|(pos, e)| (e, pos + 1))
        .collect();

    let groups = parser::parse_to_groups(contents)?;
    let sum = groups
        .iter()
        .map(|b| alphabet_scores.get(b).unwrap())
        .sum();
    Ok(sum)
}

pub fn run() -> eyre::Result<()> {
    let file_path = "../day3/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let chars = parser::parse_file(&contents)?;
    println!("Part 1 score: {}", score(chars));
    println!("Part 2 score: {}", groups_score(&contents)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_badge_score() -> eyre::Result<()> {
        let score = groups_score(INPUT);
        dbg!(&score);
        assert_eq!(score?, 70);
        Ok(())
    }

    #[test]
    fn test_the_score() -> eyre::Result<()> {
        let chars = crate::parser::parse_file(INPUT)?;
        assert_eq!(score(chars), 157);
        Ok(())
    }
}
