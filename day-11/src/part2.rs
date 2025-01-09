use std::collections::HashMap;

use anyhow::Result;

const NUM_OF_LOOPS: u8 = 75;

pub fn process(input: &str) -> Result<String> {
    let mut stones: Vec<u64> = vec![];
    for stone_str in input.split_whitespace() {
        stones.push(stone_str.parse::<u64>().unwrap());
    }

    // Took the hashmap strategy from Chris, that seems to be the largest improvement
    // compared to my part1 solution. Though I never measured along the way so I don't really know.
    let mut cache: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
        // I really like this entry().and_modify().or_insert() workflow.
        cache.entry(stone).and_modify(|n| *n += 1).or_insert(1);
    }

    for _ in 0..NUM_OF_LOOPS {
        let mut new_cache: HashMap<u64, u64> = HashMap::new();
        for (stone_val, stone_count) in cache.into_iter() {
            if stone_val == 0 {
                new_cache
                    .entry(1)
                    .and_modify(|count| *count += stone_count)
                    .or_insert(stone_count);
            } else {
                let stone_len = get_stone_len(&stone_val);
                if stone_len % 2 == 0 {
                    let (stone1, stone2) = split_stone(stone_val, &stone_len);
                    new_cache
                        .entry(stone1)
                        .and_modify(|count| *count += stone_count)
                        .or_insert(stone_count);
                    new_cache
                        .entry(stone2)
                        .and_modify(|count| *count += stone_count)
                        .or_insert(stone_count);
                } else {
                    new_cache
                        .entry(stone_val * 2024)
                        .and_modify(|count| *count += stone_count)
                        .or_insert(stone_count);
                }
            }
        }
        cache = new_cache;
    }

    Ok(cache.values().sum::<u64>().to_string())
}

// the get length and splitting functions are completely my own.
// Actually pretty proud of thinking of the math behind it.
// I really should measure the difference between my calculations and crate Chris used
fn get_stone_len(stone: &u64) -> u32 {
    let mut len = 0;
    loop {
        len += 1;
        if *stone < 10u64.pow(len) {
            break;
        }
    }
    len
}

fn split_stone(stone: u64, stone_len: &u32) -> (u64, u64) {
    let stone1 = stone / 10u64.pow(stone_len / 2);
    let stone2 = stone - 10u64.pow(stone_len / 2) * stone1;
    (stone1, stone2)
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
