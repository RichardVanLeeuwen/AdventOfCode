use anyhow::Result;
use regex::Regex;

pub fn process(input: &str, field_width: i64, field_height: i64) -> Result<String> {
    let reg = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();
    let mut quadrants: [u32; 4] = [0, 0, 0, 0];
    for line in input.lines() {
        let captures = reg.captures(line).unwrap();
        // first capture is the full match, the following ones are the actual captures
        let x_cor = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y_cor = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let x_velo = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let y_velo = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
        let x_dest = (field_width + x_cor + (100 * x_velo) % field_width) % field_width;
        let y_dest = (field_height + y_cor + (100 * y_velo) % field_height) % field_height;
        let on_left_side = x_dest < field_width / 2;
        let on_right_side = x_dest > field_width / 2;
        let on_top_half = y_dest < field_height / 2;
        let on_bottom_half = y_dest > field_height / 2;
        if on_left_side && on_top_half {
            quadrants[0] += 1;
        }
        if on_right_side && on_top_half {
            quadrants[1] += 1;
        }
        if on_left_side && on_bottom_half {
            quadrants[2] += 1;
        }
        if on_right_side && on_bottom_half {
            quadrants[3] += 1;
        }
    }

    let result: u32 = quadrants.iter().product();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input, 11, 7)?);
        Ok(())
    }
}
