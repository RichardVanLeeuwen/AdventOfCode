use std::collections::HashMap;

use anyhow::Result;
use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [
    IVec2::new(-1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, -1),
    IVec2::new(1, 1),
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

    let result: usize = positions
        .iter()
        .filter(|(_, value)| **value == 'A')
        .map(|(position, _value)| {
            let found_chars = DIRECTIONS
                .iter()
                .map(|direction| positions.get(&(position + direction)).unwrap_or(&'.'))
                .collect::<Vec<&char>>();
            let num_of_m = found_chars.iter().filter(|a| ***a == 'M').count();
            let num_of_s = found_chars.iter().filter(|a| ***a == 'S').count();
            num_of_m == 2 && num_of_s == 2 && found_chars.first() != found_chars.get(1)
        })
        .filter(|v| *v)
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
