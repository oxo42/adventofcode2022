#![allow(unused)]

use std::{collections::VecDeque, fmt::Debug};

use crate::parser::parse_input;

mod parser;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Item {
    Collection(VecDeque<Item>),
    Value(i64),
}

// impl Item {
//     fn with_slice<T>(&self, f: impl FnOnce(&[Item]) -> T) -> T {
//         match self {
//             Self::Collection(n) => f(&n[..]),
//             Self::Value(n) => f(&[Self::Value(*n)]),
//         }
//     }
// }

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

impl From<VecDeque<Item>> for Item {
    fn from(v: VecDeque<Item>) -> Self {
        Item::Collection(v)
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
    println!("Got {} pairs", pairs.len());

    let corrects = sum_corrects(pairs);
    println!("Corrects: {}", corrects);
    Ok(())
}

fn icp(Pair(left, right): Pair) -> bool {
    use Item::*;
    match (left, right) {
        (Collection(mut l_collection), Collection(mut r_collection)) => {
            let l_head = if let Some(l_head) = l_collection.pop_front() {
                l_head
            } else {
                return true;
            };
            let r_head = if let Some(r_head) = r_collection.pop_front() {
                r_head
            } else {
                return false;
            };
            match (l_head, r_head) {
                (Value(ll), Value(rr)) => {
                    if ll < rr {
                        return true;
                    }
                    if ll > rr {
                        return false;
                    }
                    // if ll == rr
                    icp(Pair::from((l_collection, r_collection)))
                }
                (ll, rr) => icp(Pair(ll, rr)),
            }
        }
        (Collection(l), Value(r)) => icp(Pair::from((Collection(l), vec![r].into()))),
        (Value(l), Collection(r)) => icp(Pair::from((vec![l].into(), Collection(r)))),
        _ => unreachable!(),
    }
}

fn sum_corrects(pairs: Vec<Pair>) -> usize {
    pairs
        .into_iter()
        .enumerate()
        .map(|(i, p)| match icp(p) {
            true => (i + 1),
            false => 0,
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
    #[test_case("[[4,4],4,4]\n[[4,4],4,4,4]\n")]
    fn test_correct_order(input: &str) {
        let pair: Pair = parse_input(input).unwrap().first().unwrap().clone();
        assert!(icp(pair));
    }

    #[test_case("[9]\n[[8,7,6]]\n")]
    #[test_case("[[[]]]\n[[]]\n")]
    #[test_case("[7,7,7,7]\n[7,7,7]\n")]
    #[test_case("[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n")]
    fn test_incorrect_order(input: &str) {
        let pair: Pair = parse_input(input).unwrap().first().unwrap().clone();
        assert!(!icp(pair));
    }

    #[test]
    fn test_sample_part_1() -> color_eyre::Result<()> {
        let pairs = parse_input(include_str!("sample.txt"))?;
        let actual = sum_corrects(pairs);
        assert_eq!(13, actual);
        Ok(())
    }
}
