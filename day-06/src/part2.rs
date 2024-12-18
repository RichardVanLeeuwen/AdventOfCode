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
    let map = input
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

    let base_path = determine_base_path(&map, &position);

    for (position_in_map, character) in base_path {
        if character != 'X' {
            continue;
        }
        let mut cloned_map = map.clone();
        cloned_map.insert(position_in_map, '#');
        if check_looping_map(cloned_map, &position) {
            result += 1;
        }
    }
    Ok(result.to_string())
}

fn check_looping_map(mut map: HashMap<IVec2, char>, start_pos: &IVec2) -> bool {
    let mut position = *start_pos;
    let mut count = 0;
    let result = 'outer: loop {
        let direction = DIRECTIONS.get((count % 4) as usize);
        'inner: loop {
            map.insert(position, 'X');
            let next_location = position + direction.unwrap();
            let character = map.get(&next_location);
            if character.is_none() {
                break 'outer false;
            }
            if character.unwrap() == &'#' {
                break 'inner;
            }
            position = next_location;
        }
        count += 1;
        if count == 200 {
            break true;
        }
    };
    result
}

fn determine_base_path(map: &HashMap<IVec2, char>, start_pos: &IVec2) -> HashMap<IVec2, char> {
    let mut cloned_map = map.clone();
    let mut position = *start_pos;
    let mut is_first = true;
    let mut count = 0u32;
    'outer: loop {
        let direction = DIRECTIONS.get((count % 4) as usize);
        if direction.is_none() {
            break;
        }
        'inner: loop {
            if is_first {
                is_first = false;
            } else {
                cloned_map.insert(position, 'X');
            }
            let next_location = position + direction.unwrap();
            let character = cloned_map.get(&next_location);
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
    cloned_map
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
