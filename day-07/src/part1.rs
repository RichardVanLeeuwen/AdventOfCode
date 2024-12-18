use std::collections::VecDeque;

use anyhow::Result;
use regex::Regex;

pub fn process(input: &str) -> Result<String> {
    let mut result = 0;
    let nums_regex = Regex::new(r"\d+").unwrap();
    for line in input.lines() {
        let matches = nums_regex.find_iter(line);
        let nums: VecDeque<u64> = matches
            .into_iter()
            .map(|num| num.as_str().parse::<u64>().unwrap())
            .collect();
        result += equatable(nums);
    }

    Ok(result.to_string())
}

fn equatable(mut numbers: VecDeque<u64>) -> u64 {
    let answer = numbers.pop_front().unwrap();
    let mut result_vec: Vec<u64> = vec![numbers.pop_front().unwrap()];
    for num in numbers {
        let mut new_vec: Vec<u64> = vec![];
        result_vec.iter().for_each(|val| {
            new_vec.push(val + num);
            new_vec.push(val * num);
        });
        result_vec = new_vec;
    }
    if result_vec.contains(&answer) {
        answer
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
