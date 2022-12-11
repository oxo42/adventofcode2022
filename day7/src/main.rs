#![allow(unused)]

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree, TreeBuilder};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Self::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    preceded(
        tag("$ "),
        alt((map(parse_cd, Into::into), map(parse_ls, Into::into))),
    )(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(s, n)| Entry::File(s, n),
    );

    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_dir, parse_file))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[derive(Debug, Default)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
}

impl FsEntry {
    fn new(size: u64, path: Utf8PathBuf) -> Self {
        Self { size, path }
    }

    fn root() -> Self {
        Self::new(0, "/".into())
    }
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // let lines: Vec<_> = INPUT
        let lines: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| (all_consuming(parse_line)(l)).finish().unwrap().1)
        .collect();
    // for line in &lines {
    //     println!("{line:?}");
    // }

    let mut tree: Tree<FsEntry> = TreeBuilder::new().with_node_capacity(5).build();
    let root_id = tree.insert(Node::new(FsEntry::root()), InsertBehavior::AsRoot)?;
    let mut cwd = root_id;
    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => (),
                Command::Cd(dir) => match dir.as_str() {
                    "/" => {}
                    ".." => cwd = tree.get(&cwd)?.parent().unwrap().clone(),
                    _ => {
                        let node = Node::new(FsEntry::new(0, dir));
                        cwd = tree.insert(node, InsertBehavior::UnderNode(&cwd))?;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {}
                Entry::File(size, name) => {
                    let node = Node::new(FsEntry::new(size, name));
                    tree.insert(node, InsertBehavior::UnderNode(&cwd))?;
                }
            },
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{s}");

    let sum: u64 = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        // .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum();
    dbg!(sum);

    let total_space = 70_000_000_u64;
    let used = total_size(&tree, tree.get(tree.root_node_id().unwrap())?)?;
    let free_space = total_space - dbg!(used);
    let needed_free_space = 30_000_000_u64;
    let min_delta = needed_free_space - free_space;

    let smallest: u64 = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s > min_delta)
        .min()
        .unwrap();

    dbg!(smallest);

    Ok(())
}

const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

#[cfg(test)]
mod tests {
    use super::*;
}
