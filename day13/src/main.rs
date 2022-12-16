#![allow(unused)]

use std::{collections::VecDeque, fmt::Debug};

use crate::parser::parse_input;

mod parser;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Item {
    Collection(VecDeque<Item>),
    Value(i64),
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Collection(arg0) => f.debug_list().entries(arg0.iter()).finish(),
            Self::Value(arg0) => write!(f, "{}", arg0),
        }
    }
}

impl From<Vec<i64>> for Item {
    fn from(v: Vec<i64>) -> Self {
        Item::Collection(v.into_iter().map(Into::into).collect())
    }
}

impl From<i64> for Item {
    fn from(i: i64) -> Self {
        Item::Value(i)
    }
}

impl From<Vec<Item>> for Item {
    fn from(v: Vec<Item>) -> Self {
        Item::Collection(VecDeque::from(v))
    }
}

#[derive(Debug, Clone)]
struct Pair(Item, Item);

impl From<(Item, Item)> for Pair {
    fn from((left, right): (Item, Item)) -> Self {
        Pair(left, right)
    }
}

impl From<(VecDeque<Item>, VecDeque<Item>)> for Pair {
    fn from((left, right): (VecDeque<Item>, VecDeque<Item>)) -> Self {
        Pair(Item::Collection(left), Item::Collection(right))
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");
    let pairs = parse_input(input)?;
    pairs.iter().for_each(|p| println!("{:?}", p));

    let corrects = sum_corrects(pairs);
    println!("Corrects: {}", corrects);
    Ok(())
}

fn is_correct_order(Pair(left, right): Pair) -> bool {
    // dbg!(&left, &right);
    use Item::*;
    match (left, right) {
        (Collection(l), Value(r)) => {
            let mut new_pair = Pair::from((Collection(l), vec![r].into()));
            is_correct_order(new_pair)
        }
        (Value(l), Collection(r)) => {
            let new_pair = Pair::from((vec![l].into(), Collection(r)));
            is_correct_order(new_pair)
        }
        (Collection(mut l), Collection(mut r)) => {
            let l_head = l.pop_front();
            if l_head.is_none() {
                return true;
            }
            let r_head = r.pop_front();
            if r_head.is_none() {
                return false;
            }
            let l_head = l_head.unwrap();
            let r_head = r_head.unwrap();
            let tail_pair = Pair::from((l, r));
            match (l_head, r_head) {
                (Value(l), Value(r)) => {
                    // dbg!(&l, &r);
                    if l < r {
                        true
                    } else if l > r {
                        false
                    } else {
                        is_correct_order(tail_pair)
                    }
                }
                (l, r) => is_correct_order(Pair(l, r)) && is_correct_order(tail_pair),
            }
        }

        _ => todo!(),
    }
}

fn sum_corrects(pairs: Vec<Pair>) -> usize {
    pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| match is_correct_order(p) {
            true => {
                println!("{}", i + 1);
                Some(i + 1)
            }
            false => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use Item::*;

    #[test_case("[1,1,3,1,1]\n[1,1,5,1,1]\n")]
    #[test_case("[]\n[3]\n")]
    #[test_case("[[1],[2,3,4]]\n[[1],4]\n")]
    fn test_correct_order(input: &str) {
        let pair: Pair = parse_input(input).unwrap().first().unwrap().clone();
        assert!(is_correct_order(pair));
    }

    #[test_case("[[[]]]\n[[]]\n")]
    #[test_case("[9]\n[[8,7,6]]\n")]
    fn test_incorrect_order(input: &str) {
        let pair: Pair = parse_input(input).unwrap().first().unwrap().clone();
        assert!(!is_correct_order(pair));
    }

    #[test]
    fn test_sample_part_1() -> color_eyre::Result<()> {
        let pairs = parse_input(include_str!("sample.txt"))?;
        let actual = sum_corrects(pairs);
        assert_eq!(13, actual);
        Ok(())
    }
}
