use std::{collections::HashMap, fs};

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

            vec.sort_unstable();
            vec.dedup();
            vec.len()
        })
        .into_iter()
        .sum()
}

pub fn part2() -> usize {
    do_part2(&&get_day_06_input())
}

fn do_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|f| {
            let person_count = f.lines().count();
            let mut char_count = HashMap::new();
            for person_result in f.lines() {
                for char in person_result.chars() {
                    let count = char_count.entry(char).or_insert(0);
                    *count += 1;
                }
            }
            let mut all_persons_agree_on = 0;
            for (_char, count) in char_count {
                if count == person_count {
                    all_persons_agree_on += 1;
                }
            }
            all_persons_agree_on
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
