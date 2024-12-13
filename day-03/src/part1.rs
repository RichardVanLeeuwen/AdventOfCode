use anyhow::Result;
use regex::Regex;

pub fn process(input: &str) -> Result<String> {
    let mut result: i32 = 0;
    let mul_reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mul_matches = mul_reg.captures_iter(input);
    for capture in mul_matches {
        let product: i32 = capture
            .iter()
            .map(|num| num.unwrap().as_str().parse::<i32>().unwrap_or(1))
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
