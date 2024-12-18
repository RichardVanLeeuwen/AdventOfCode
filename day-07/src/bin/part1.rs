use day_07::part1::process;
use anyhow::Result;

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}