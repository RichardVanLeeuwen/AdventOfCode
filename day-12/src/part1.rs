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
    let mut map: HashMap<IVec2, char> = HashMap::new();
    let mut map_height: u8 = 0;
    let mut map_width: u8 = 0;
    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            map_width = line.len() as u8;
        };
        map_height += 1;
        for (x, character) in line.chars().enumerate() {
            if x == 0 {};
            let location = IVec2::new(x as i32, y as i32);
            map.insert(location, character);
        }
    }

    let mut result = 0;
    for y in 0..map_height {
        for x in 0..map_width {
            let location = IVec2::new(x as i32, y as i32);
            if map.contains_key(&location) {
                result += calculate_location(location, &mut map);
            }
        }
    }

    Ok(result.to_string())
}

fn calculate_location(location: IVec2, map: &mut HashMap<IVec2, char>) -> u32 {
    let mut perimeter: u32 = 0;
    let mut area: u32 = 0;
    let region_character = *map.get(&location).unwrap();
    let mut region: Vec<IVec2> = vec![];
    check_neighbors(
        &mut perimeter,
        &mut area,
        &mut region,
        &region_character,
        map,
        location,
    );
    for location in region {
        map.remove(&location);
    }

    area * perimeter
}

fn check_neighbors(
    perimeter: &mut u32,
    area: &mut u32,
    region: &mut Vec<IVec2>,
    region_character: &char,
    map: &mut HashMap<IVec2, char>,
    location: IVec2,
) {
    *area += 1;
    region.push(location);
    for direction in DIRECTIONS {
        let next_location = location + direction;
        let next_value = map.get(&next_location);
        if next_value.is_none() || next_value.unwrap() != region_character {
            *perimeter += 1;
            continue;
        } else if !region.contains(&next_location) {
            check_neighbors(
                perimeter,
                area,
                region,
                region_character,
                map,
                next_location,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!("140", process(input)?);
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("772", process(input)?);
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
