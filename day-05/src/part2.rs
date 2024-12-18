use std::{cmp::Ordering, collections::HashMap};

use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    // split input into rules and updates
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let rules = split_input[0];

    // put rules into hashmap
    let mut mapped_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in rules.lines() {
        // protection
        if line.is_empty() {
            continue;
        };
        // separate the numbers and parse them into u32
        let split_numbers: Vec<u32> = line
            .split('|')
            // filter_map with ok() ignores any parsing errors
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();
        mapped_rules
            .entry(split_numbers[0])
            // if entry exists we append the number to the vector
            .and_modify(|vector| vector.push(split_numbers[1]))
            // if it doesn't exists we make a new vector with the number
            .or_insert(vec![split_numbers[1]]);
    }

    let mut result: u32 = 0;
    // process updates
    let updates = split_input[1];
    for line in updates.lines() {
        // split numbers and map them
        let numbers: Vec<u32> = line
            .split(',')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();
        if !check_sorted(&numbers, &mapped_rules) {
            let sorted_numbers = sort_by_rules(numbers, &mapped_rules);
            result += sorted_numbers[sorted_numbers.len() / 2];
        }
    }

    Ok(result.to_string())
}

fn sort_by_rules(mut vec_of_numbers: Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    vec_of_numbers.sort_by(|a, b| {
        let rules = rules.get(a);
        if let Some(rule) = rules {
            if rule.contains(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Equal
        }
    });
    vec_of_numbers
}

fn check_sorted(vec_of_numbers: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut collector: Vec<u32> = vec![];
    // named loop, so we can break this loop from inside a loop in this loop
    'out: for number in vec_of_numbers {
        let rules = rules.get(number);
        if let Some(rule) = rules {
            // if the current number we are checking has a number in it's hashmap vec
            // and that number is also in the collector vec
            // then that number had to go after, but came before, so a rule was broken
            for under_check in &collector {
                if rule.contains(under_check) {
                    break 'out;
                }
            }
        };
        collector.push(*number);
    }
    collector.len() == vec_of_numbers.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
