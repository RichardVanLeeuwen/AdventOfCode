use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let mut drive: Vec<i32> = vec![];
    // input is a single line, so not using lines() for this one
    for (index, character) in input.chars().enumerate() {
        let num = character.to_digit(10);
        if num.is_none() {
            continue;
        }
        for _ in 0..num.unwrap() {
            if index % 2 == 0 {
                drive.push((index / 2) as i32);
            } else {
                drive.push(-1);
            }
        }
    }

    let mut result = 0u128;
    let mut count = 0;
    let drive_length = &(drive.len() as i32);

    for (index, number) in drive.iter().enumerate() {
        let mut temp_number = number;
        while temp_number == &-1 {
            temp_number = drive.get((drive_length - count - 1) as usize).unwrap();
            count += 1;
        }

        result += (index as i32 * temp_number) as u128;
        if *drive_length == (index as i32) + count + 1 {
            break;
        };
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
