#![allow(unused)]
mod parser;

use eyre::Report;
use std::env;
use std::fs;

pub fn day1_part1() {
    let file_path = "../day1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    match parser::parse_whole_file(&contents) {
        Ok((_, mut elves)) => {
            elves.sort();
            elves.reverse();
            println!("Most calorific elf: {:?}", elves.iter().max().unwrap());
            let top3: i64 = elves.iter().take(3).sum();
            println!("Top 3 elves calories: {:}", top3);
        }
        Err(e) => eprintln!("{:?}", e),
    };
}
