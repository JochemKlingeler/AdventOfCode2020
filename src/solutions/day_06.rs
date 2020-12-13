use std::fs;

pub fn part1() -> usize {
    do_part1(&get_day_06_input())
}

fn do_part1(_input: &str) -> usize {
    0
}

pub fn part2() -> usize {
    do_part2(&get_day_06_input())
}

fn do_part2(_input: &str) -> usize {
    0
}

fn get_day_06_input() -> String {
    fs::read_to_string("./input/day_06.txt").expect("Something went wrong reading the file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_demo() {
        assert_eq!(2, 2);
    }
    #[test]
    fn part2_demo() {
        assert_eq!(2, 2);
    }
}
