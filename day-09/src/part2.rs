use anyhow::Result;

struct DriveSector {
    free: bool,
    data: Vec<u32>,
    sector_length: u8,
}
impl DriveSector {
    fn new(free: bool, data: Vec<u32>, sector_length: u8) -> DriveSector {
        DriveSector {
            free,
            data,
            sector_length,
        }
    }
}

pub fn process(input: &str) -> Result<String> {
    let mut drive: Vec<DriveSector> = vec![];
    // input is a single line, so not using lines() for this one
    for (index, character) in input.chars().enumerate() {
        let num = character.to_digit(10);
        if num.is_none() {
            continue;
        }
        let unwrapped_num = num.unwrap();
        for _ in 0..unwrapped_num {
            drive.push(DriveSector::new(
                index % 2 == 0,
                if index % 2 == 0 {
                    vec![unwrapped_num; unwrapped_num as usize]
                } else {
                    vec![]
                },
                unwrapped_num as u8,
            ));
        }
    }

    let mut result = 0u128;
    let mut count = 0;
    let drive_length = &(drive.len() as u32);

    for (index, sector) in drive.iter().enumerate().rev() {
        let mut loop_count = 0;
        loop {
            let attempt_sector = drive.get(loop_count).unwrap();
            if !attempt_sector.free {
                continue;
            }
            if attempt_sector.sector_length >= sector.sector_length {
                todo!()
            }
        }

        // result += (index as u32 * temp_number) as u128;
        if *drive_length == (index as u32) + count + 1 {
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
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
