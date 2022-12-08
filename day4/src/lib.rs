mod parser;

use parser::ContainsExt;

pub fn run() -> eyre::Result<()> {
    let input = include_str!("../input.txt");
    let pairs = parser::parse_file(input)?;
    let count = pairs
        .iter()
        .filter(|(a, b)| a.contains_or_is_contained(b))
        .count();

    let overlaps = pairs
        .iter()
        .filter(|(a, b)| a.overlaps_or_is_overlapped(b))
        .count();

    println!("Day 4: Part 1: {count}");
    println!("Day 4: Part 2: {overlaps}");
    Ok(())
}

#[cfg(test)]
mod tests {
    pub const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
}
