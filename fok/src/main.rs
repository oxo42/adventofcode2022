use std::time::Instant;

use day1::day1_part1;

fn main() -> eyre::Result<()> {
    let start = Instant::now();
    day1_part1();
    let duration = Instant::now().duration_since(start);
    println!("It took {:?}", duration);
    Ok(())
}
