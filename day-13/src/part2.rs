use anyhow::Result;
use regex::Regex;

pub fn process(input: &str) -> Result<String> {
    let mut result = 0;
    let numbers_regex = Regex::new(r"\d+").unwrap();
    for set_of_lines in input.split("\n\n") {
        let machine: Vec<Vec<u128>> = set_of_lines
            .lines()
            .map(|line| {
                numbers_regex
                    .find_iter(line)
                    .map(|num| num.as_str().parse::<u128>().unwrap())
                    .collect::<Vec<u128>>()
            })
            .collect();
        // stupidity, but don't know how else to solve this.
        // Apparently, this is still good enough to compile and run in under a second on a cheap 10
        // year old laptop
        #[allow(non_snake_case)]
        let A_x_translation = machine.get(0).unwrap().get(0).unwrap();
        let A_y_translation = machine.get(0).unwrap().get(1).unwrap();
        let B_x_translation = machine.get(1).unwrap().get(0).unwrap();
        let B_y_translation = machine.get(1).unwrap().get(1).unwrap();
        let x_prize = machine.get(2).unwrap().get(0).unwrap() + 10000000000000;
        let y_prize = machine.get(2).unwrap().get(1).unwrap() + 10000000000000;

        let mut A: u128 = 0;
        let mut B: u128 = 0;
        'search_loop: loop {
            A += 1;
            loop {
                B += 1;
                let attempt_x = A_x_translation * A + B_x_translation * B;
                let attempt_y = A_y_translation * A + B_y_translation * B;
                if attempt_x > x_prize && attempt_y > y_prize {
                    break 'search_loop;
                }
                if attempt_x > x_prize {
                    continue 'search_loop;
                }
                if attempt_y > y_prize {
                    continue 'search_loop;
                }
                if attempt_x == x_prize && attempt_y == y_prize {
                    result += A * 3 + B;
                    break 'search_loop;
                }
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
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
