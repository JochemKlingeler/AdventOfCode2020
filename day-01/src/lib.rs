use std::fs;

pub fn part1() -> u32 {
    do_part1(&get_day_01_input())
}

pub fn part2() -> u32 {
    do_part2(&get_day_01_input())
}

fn do_part1(input: &[u32]) -> u32 {
    for a in input {
        for b in input {
            if 2020 == (a + b) {
                return a * b;
            }
        }
    }
    panic!("No valid values!");
}

pub fn do_part2(input: &[u32]) -> u32 {
    for a in input {
        for b in input {
            for c in input {
                if 2020 == (a + b + c) {
                    return a * b * c;
                }
            }
        }
    }
    panic!("No valid values!");
}

fn get_day_01_input() -> Vec<u32> {
    fs::read_to_string("./input/day_01.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse::<u32>().expect("All values should be u32s"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_demo() {
        assert_eq!(514579, do_part1(&make_demo_input()));
    }
    #[test]
    fn part2_demo() {
        assert_eq!(241861950, do_part2(&make_demo_input()));
    }

    fn make_demo_input() -> Vec<u32> {
        return vec![1721, 979, 366, 299, 675, 1456];
    }
}
