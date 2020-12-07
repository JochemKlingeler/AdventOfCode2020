pub fn part1(input: &[u32]) -> Result<u32, String> {
    for a in input {
        for b in input {
            if 2020 == (a + b) {
                return Ok(a * b);
            }
        }
    }
    Err(String::from("No valid values!"))
}

pub fn part2(input: &[u32]) -> Result<u32, String> {
    for a in input {
        for b in input {
            for c in input {
                if 2020 == (a + b + c) {
                    return Ok(a * b * c);
                }
            }
        }
    }
    Err(String::from("No valid values!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_demo() {
        let result = part1(&make_demo_input());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 514579);
    }
    #[test]
    fn part2_demo() {
        let result = part2(&make_demo_input());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 241861950);
    }

    fn make_demo_input() -> Vec<u32> {
        return vec![1721, 979, 366, 299, 675, 1456];
    }
}
