use anyhow::{Context, Result};
use day_03::part1::process;

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
