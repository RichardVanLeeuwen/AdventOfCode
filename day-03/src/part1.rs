use anyhow::Result;
use regex::Regex;

pub fn process(input: &str) -> Result<String> {
    let mut result: i32 = 0;
    let mul_reg = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let num_regex = Regex::new(r"\d+").unwrap();
    let mul_matches = mul_reg.find_iter(input);
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
