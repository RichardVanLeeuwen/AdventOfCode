use anyhow::Result;
use glam::IVec2;
use std::{char, collections::HashMap};

pub fn process(input: &str) -> Result<String> {
    let mut map: HashMap<char, Vec<IVec2>> = HashMap::new();
    let mut map_length = 0i32;
    let map_height = input.lines().count() as i32;
    for (y, line) in input.lines().enumerate() {
        if y == 1 {
            map_length = line.len() as i32;
        }
        for (x, character) in line.chars().enumerate() {
            if character == '.' {
                continue;
            }
            let map_contents = map.get_mut(&character);
            if let Some(map_contents) = map_contents {
                map_contents.push(IVec2::new(x as i32, y as i32));
            } else {
                map.insert(character, vec![IVec2::new(x as i32, y as i32)]);
            }
        }
    }

    let mut antinode_locations: Vec<IVec2> = vec![];
    for (_, locations) in map {
        if locations.len() <= 1 {
            continue;
        }
        for location_1 in locations.iter() {
            for location_2 in locations.iter() {
                if location_1 == location_2 {
                    continue;
                }
                let antinode_location = location_1 - location_2 + location_1;
                if antinode_location.y > map_height || antinode_location.y < 0i32 {
                    continue;
                }
                if antinode_location.x > map_length || antinode_location.x < 0i32 {
                    continue;
                }
                if !antinode_locations.contains(&antinode_location) {
                    antinode_locations.push(antinode_location);
                }
            }
        }
    }

    Ok(antinode_locations.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
