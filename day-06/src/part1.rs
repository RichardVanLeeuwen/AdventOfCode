use anyhow::Result;
use glam::IVec2;
use std::collections::HashMap;

const DIRECTIONS: [IVec2; 4] = [
    IVec2::new(0, -1),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
];

pub fn process(input: &str) -> Result<String> {
    let mut result = 0;
    let mut position: IVec2 = IVec2::new(0, 0);
    // create the map
    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, character)| (IVec2::new(x as i32, y as i32), character))
        })
        .collect::<HashMap<IVec2, char>>();

    for (pos, character) in &map {
        if character == &'^' {
            position = *pos;
        }
    }

    // start looping over DIRECTIONS
    let mut count = 0u32;
    'outer: loop {
        let direction = DIRECTIONS.get((count % 4) as usize);
        if direction.is_none() {
            break;
        }
        'inner: loop {
            map.insert(position, 'X');
            let next_location = position + direction.unwrap();
            let character = map.get(&next_location);
            if character.is_none() {
                break 'outer;
            }
            if character.unwrap() == &'#' {
                break 'inner;
            }
            position = next_location;
        }
        count += 1;
    }

    for (_, character) in map {
        if character == 'X' {
            result += 1;
        }
    }
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
