use anyhow::Result;
use glam::IVec2;
use regex::Regex;

const SHAPE: [IVec2; 8] = [
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
    IVec2::new(-2, -2),
    IVec2::new(-1, -2),
    IVec2::new(0, -2),
    IVec2::new(1, -2),
    IVec2::new(2, -2),
];

struct Robot {
    x_cor: i32,
    y_cor: i32,
    x_velo: i32,
    y_velo: i32,
}

pub fn process(input: &str, field_width: i32, field_height: i32) -> Result<String> {
    let reg = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let captures = reg.captures(line).unwrap();
        // first capture is the full match, the following ones are the actual captures
        let x_cor = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y_cor = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let x_velo = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let y_velo = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
        robots.push(Robot {
            x_cor,
            y_cor,
            x_velo,
            y_velo,
        });
    }
    let mut result = 0;
    'check_robots: loop {
        let mut locations: Vec<IVec2> = vec![];
        for robot in robots.iter() {
            let x_dest =
                (field_width + robot.x_cor + (result * robot.x_velo) % field_width) % field_width;
            let y_dest = (field_height + robot.y_cor + (result * robot.y_velo) % field_height)
                % field_height;
            locations.push(IVec2::new(x_dest, y_dest));
        }
        for location in locations.iter() {
            if SHAPE
                .iter()
                .all(|vector_to_check| locations.contains(&(location + vector_to_check)))
            {
                break 'check_robots;
            }
        }
        result += 1;
    }

    Ok(result.to_string())
}
