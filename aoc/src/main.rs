use std::time::Instant;

fn main() -> eyre::Result<()> {
    let start = Instant::now();
    // day1::day1_part1();
    day2::run()?;
    let duration = Instant::now().duration_since(start);
    println!("It took {:?}", duration);
    Ok(())
}
