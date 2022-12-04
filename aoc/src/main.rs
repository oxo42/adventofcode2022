use std::time::Instant;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    color_backtrace::install();

    let start = Instant::now();
    // day1::day1_part1();
    day3::run()?;
    let duration = Instant::now().duration_since(start);
    println!("It took {:?}", duration);
    Ok(())
}
