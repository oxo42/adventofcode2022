#![allow(unused)]
mod game;
mod parser;

use std::fs;

pub fn run() -> eyre::Result<()> {
    let file_path = "../day2/input.txt";
    let games = parser::games(file_path)?;
    let score: i64 = games.iter().map(|g| g.score()).sum();
    println!("Part 1 score is {score}");
    let part2_score: i64 = games.iter().map(|g| g.part2_score()).sum();
    println!("Part 2 score is {part2_score}");
    Ok(())
}
