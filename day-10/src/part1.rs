use std::collections::HashMap;

use anyhow::Result;
use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [
    IVec2::new(0, -1),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
];

pub fn process(input: &str) -> Result<String> {
    let mut map: HashMap<IVec2, u8> = HashMap::new();
    let mut trailheads: Vec<IVec2> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            let number = character.to_digit(10).unwrap();
            let location = IVec2::new(x as i32, y as i32);
            map.insert(location, number as u8);
            if number == 0 {
                trailheads.push(location);
            }
        }
    }

    let mut result = 0;
    for trailhead in trailheads {
        let trail_ends = calc_next_step(1, vec![trailhead], &map);
        result += trail_ends.len();
    }
    Ok(result.to_string())
}

fn calc_next_step(height: u8, possible_steps: Vec<IVec2>, map: &HashMap<IVec2, u8>) -> Vec<IVec2> {
    let mut next_steps: Vec<IVec2> = vec![];
    for location in possible_steps {
        for direction in DIRECTIONS {
            let next_location = direction + location;
            let possible_next_location_value = map.get(&next_location);
            if let Some(next_location_value) = possible_next_location_value {
                if height == *next_location_value && !next_steps.contains(&next_location) {
                    next_steps.push(direction + location);
                }
            }
        }
    }
    if height == 9 {
        next_steps
    } else {
        calc_next_step(height + 1, next_steps, map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
