use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let mut result: i32 = 0;
    for line in input.lines() {
        if check_safety(line) {
            result += 1;
        } else {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            for index in 0..numbers.len() {
                let mut new_numbers = numbers.clone();
                new_numbers.remove(index);
                if check_safety_from_vec(new_numbers) {
                    result += 1;
                    break;
                }
            }
        }
    }

    Ok(result.to_string())
}

fn check_safety(line: &str) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    check_safety_from_vec(numbers)
}

fn check_safety_from_vec(numbers: Vec<i32>) -> bool {
    let mut differences: Vec<i32> = vec![];
    let mut un_safe = false;
    for window in numbers.windows(2) {
        let cur_diff = window[0] - window[1];
        if (1..=3).contains(&cur_diff.abs()) {
            differences.push(cur_diff)
        } else {
            un_safe = true;
        }
    }
    differences.iter().sum::<i32>().abs() == differences.iter().map(|num| num.abs()).sum()
        && !un_safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
