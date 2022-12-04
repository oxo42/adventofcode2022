use std::collections::HashSet;

use nom::{
    character::complete::{alpha1, newline},
    combinator::map,
    multi::{many0, many1, many_m_n},
    sequence::terminated,
    IResult,
};

fn line(input: &str) -> IResult<&str, HashSet<char>> {
    map(alpha1, |s: &str| {
        let (left, right) = s.split_at(s.len() / 2);
        let left_set: HashSet<char> = left.chars().collect();
        right.chars().filter(|r| left_set.contains(r)).collect()
    })(input)
}

fn whole_file(input: &str) -> IResult<&str, Vec<HashSet<char>>> {
    many1(terminated(line, newline))(input)
}

fn filter_out_badge(groups: Vec<&str>) -> char {
    *groups
        .into_iter()
        .map(|g| g.chars().collect::<HashSet<_>>())
        .reduce(|accum, group| {
            let a = accum.into_iter().filter(|x| group.contains(x)).collect();
            a
        })
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

fn one_group(input: &str) -> IResult<&str, char> {
    map(
        many_m_n(3, 3, terminated(alpha1, newline)),
        filter_out_badge,
    )(input)
}

fn groups(input: &str) -> IResult<&str, Vec<char>> {
    many0(one_group)(input)
}

pub fn parse_to_groups(contents: &str) -> eyre::Result<Vec<char>> {
    match groups(contents) {
        Ok((_, groups)) => Ok(groups),
        Err(e) => {
            eprintln!("Error: {e:?}");
            Err(eyre::eyre!("Went boom: {e:?}"))
        }
    }
}

fn flatten(sets: Vec<HashSet<char>>) -> Vec<char> {
    let mut v = Vec::new();
    for s in sets {
        v.extend(s.into_iter());
    }
    v
}

pub fn parse_file(contents: &str) -> eyre::Result<Vec<char>> {
    match whole_file(contents) {
        Ok((_, sets)) => Ok(flatten(sets)),
        Err(e) => {
            eprintln!("Error: {e:?}");
            Err(eyre::eyre!("Went boom"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::INPUT;

    #[test]
    fn check_one_group() -> eyre::Result<()> {
        let (rem, badge) = one_group(INPUT)?;
        assert!(rem.starts_with("wMqv"));
        assert_eq!(badge, 'r');
        Ok(())
    }

    #[test]
    fn test_filter_badge() {
        let groups = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];
        let badge = filter_out_badge(groups);
        assert_eq!('r', badge);
    }

    #[test]
    fn check_groups() -> eyre::Result<()> {
        let (_, groups) = groups(INPUT)?;
        assert_eq!(groups.len(), 2);
        Ok(())
    }
    #[test]
    fn test_lines_work() -> eyre::Result<()> {
        let (remaining, set) = line(INPUT)?;
        assert!(remaining.starts_with("\njqHRN"));
        assert_eq!(HashSet::from(['p']), set);
        Ok(())
    }

    #[test]
    fn lines_all_work() -> eyre::Result<()> {
        let sets = whole_file(INPUT)?;
        let expected = vec![
            HashSet::from(['p']),
            HashSet::from(['L']),
            HashSet::from(['P']),
            HashSet::from(['v']),
            HashSet::from(['t']),
            HashSet::from(['s']),
        ];
        assert_eq!(expected, sets.1);
        Ok(())
    }
}
