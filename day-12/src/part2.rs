use std::collections::HashMap;

use anyhow::Result;
use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [
    IVec2::new(0, -1),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
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
        None,
    );
    println!("region_character: {}", region_character);
    println!("area: {}", area);
    println!("perimeter: {}", perimeter);
    for location in region {
        println!("location: {}", location);
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
    previous_direction_index: Option<usize>,
) {
    *area += 1;
    region.push(location);
    for direction in DIRECTIONS {
        let next_location = location + direction;
        let next_value = map.get(&next_location);
        let current_direction_index = DIRECTIONS.iter().position(|d| d == &direction).unwrap();
        if next_value.is_none() || next_value.unwrap() != region_character {
            // this is broken because we only loop through all locations once.
            // In single height corridors this means it is evaluated in one direction, but then
            // never going back, so 1 side remains unevaluated
            if !is_not_in_same_direction(previous_direction_index, current_direction_index) {
                *perimeter += 1;
            }
            continue;
        } else if !region.contains(&next_location) {
            check_neighbors(
                perimeter,
                area,
                region,
                region_character,
                map,
                next_location,
                Some(current_direction_index),
            );
        }
    }
}

fn is_not_in_same_direction(
    previous_direction_index: Option<usize>,
    current_direction_index: usize,
) -> bool {
    if previous_direction_index.is_none() {
        true
    } else if previous_direction_index.unwrap() as i32 - current_direction_index as i32 == -1 {
        false
    } else {
        !(previous_direction_index.unwrap() == 3 && current_direction_index == 0)
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
        assert_eq!("80", process(input)?);
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("436", process(input)?);
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!("236", process(input)?);
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!("368", process(input)?);
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
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
