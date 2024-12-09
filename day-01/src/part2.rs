use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    let mut result: i32 = 0;
    for number in &left {
        for num in &right {
            if num == number {
                result += number;
            }
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
