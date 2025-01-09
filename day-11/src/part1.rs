use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let mut stones: Vec<u128> = vec![];
    for stone_str in input.split_whitespace() {
        stones.push(stone_str.parse::<u128>().unwrap());
    }
    for _ in 0..25 {
        let mut temp_stones_vec: Vec<u128> = vec![];
        for stone in stones {
            if stone == 0 {
                temp_stones_vec.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let stone1 = stone_str[..stone_str.len() / 2].parse::<u128>().unwrap();
                let stone2 = stone_str[stone_str.len() / 2..].parse::<u128>().unwrap();
                temp_stones_vec.push(stone1);
                temp_stones_vec.push(stone2);
            } else {
                temp_stones_vec.push(stone * 2024);
            }
        }
        stones = temp_stones_vec;
    }

    let result = stones.len();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
