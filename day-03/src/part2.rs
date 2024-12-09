use anyhow::Result;
use regex::Regex;

pub fn process(input: &str) -> Result<String> {
    let do_regex = Regex::new(r"(?ms)do()(.*)").unwrap();
    let dont_parts = input.split("don't()");
    let mut active_input = "".to_owned();
    for (index, part) in dont_parts.enumerate() {
        if index == 0 {
            active_input.push_str(part);
            continue;
        }
        let unsure_captures = do_regex.captures(part);
        if let Some(capture) = unsure_captures {
            active_input.push_str(&capture[0])
        }
    }

    let mut result: i32 = 0;
    let mul_reg = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let num_regex = Regex::new(r"\d+").unwrap();
    let mul_matches = mul_reg.find_iter(&active_input);
    for mul_match in mul_matches {
        let product: i32 = num_regex
            .find_iter(mul_match.as_str())
            .map(|num| num.as_str().parse::<i32>().unwrap())
            .product();
        result += product;
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
