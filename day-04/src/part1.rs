use std::{char, collections::HashMap};

use anyhow::Result;
use glam::IVec2;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
];

pub fn process(input: &str) -> Result<String> {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>();

    let mas = ['M', 'A', 'S'];
    let result: usize = positions
        .iter()
        .filter(|(_, value)| **value == 'X')
        .map(|(position, _value)| {
            DIRECTIONS
                .iter()
                .map(|direction| {
                    let mut all_true = true;
                    for (index, offset) in direction.iter().enumerate() {
                        if positions.get(&(position + offset)) != mas.get(index) {
                            all_true = false;
                            break;
                        }
                    }
                    all_true
                })
                .filter(|b| *b)
                .count()
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
