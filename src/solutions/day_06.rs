use std::collections::HashMap;
use std::fs;

pub fn part1() -> usize {
    do_part1(&get_day_06_input())
}

fn do_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|f| {
            let mut vec = f
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>();

            vec.sort();
            vec.dedup();
            vec.len()
        })
        .into_iter()
        .sum()
}

pub fn part2() -> usize {
    do_part2(&get_day_06_input())
}

fn do_part2(_input: &str) -> usize {
    // Idea, in the hashmap we store how many times each char was given
    // and then count the items that have a count the same as linecount.
    // This way we know all passengers have given the same answers.
    input
        .split("\n\n")
        .map(|f| {
            let mut char_count = HashMap::new();
            let lines = f.lines();
            for line in lines {
                line.chars()
            }
        })
        .into_iter()
        .sum()
}

fn get_day_06_input() -> String {
    fs::read_to_string("./input/day_06.txt").expect("Something went wrong reading the file")
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part1_demo() {
        assert_eq!(11, do_part1(DEMO));
    }
    #[test]
    fn part2_demo() {
        assert_eq!(6, do_part2(DEMO));
    }
}
