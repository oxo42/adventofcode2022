#![allow(unused)]

use std::{cmp::Ordering, collections::VecDeque, fmt::Debug};

use itertools::Itertools;
use serde::Deserialize;

#[derive(PartialEq, Eq, Clone, Deserialize)]
#[serde(untagged)]
enum Node {
    Collection(Vec<Node>),
    Value(i64),
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::Collection(n) => f(&n[..]),
            Self::Value(n) => f(&[Self::Value(*n)]),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Collection(arg0) => f.debug_list().entries(arg0.iter()).finish(),
            Self::Value(arg0) => write!(f, "{}", arg0),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Node::*;
        match (self, other) {
            (Value(a), Value(b)) => a.partial_cmp(b),
            (l, r) => Some(l.with_slice(|l| {
                r.with_slice(|r| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(aa, bb)| aa.cmp(bb))
                        .find(|&ord| ord != Ordering::Equal)
                        .unwrap_or_else(|| l.len().cmp(&r.len()))
                })
            })),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part1() {
    let input = include_str!("input.txt");
    let sum: usize = input
        .split("\n\n")
        .enumerate()
        .map(|(i, groups)| {
            let (l, r) = groups
                .lines()
                .map(|line| serde_json::from_str::<Node>(line).unwrap())
                .collect_tuple()
                .unwrap();
            if l < r {
                i + 1
            } else {
                0
            }
        })
        .sum();
    println!("Sum is {sum}");
}

fn main() {
    let input = include_str!("input.txt");
    let dividers = vec![
        Node::Collection(vec![Node::Value(2)]),
        Node::Collection(vec![Node::Value(6)]),
    ];

    let mut packets = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| serde_json::from_str::<Node>(line).unwrap())
        .chain(dividers.iter().cloned())
        .collect_vec();
    packets.sort();

    let decoder_key: usize = dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product();
    dbg!(decoder_key);
}
