use anyhow::Result;
use day_09::part1::process;

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
