use std::convert::TryInto;

pub fn part1(input: &Vec<String>) -> usize {
    input.iter().filter(|i| part_1_is_valid(&i)).count()
}

pub fn part2(input: &Vec<String>) -> usize {
    input.iter().filter(|i| part_2_is_valid(&i)).count()
}

fn part_1_is_valid(input: &String) -> bool {
    let (min, max, char, password) = split_string_into_results(input);
    let char_count: usize = password
        .chars()
        .filter(|c| c == &char)
        .count()
        .try_into()
        .unwrap();
    min <= char_count && char_count <= max
}

fn part_2_is_valid(input: &String) -> bool {
    let (min, max, char, password) = split_string_into_results(input);
    let char1: char = password.chars().nth(min - 1).unwrap();
    let char2: char = password.chars().nth(max - 1).unwrap();
    (char1 == char && char2 != char) || (char2 == char && char1 != char)
}

fn split_string_into_results(input: &String) -> (usize, usize, char, &str) {
    let mut main_split = input.splitn(3, ' ');
    let (rules, char_to_find, password) = (
        main_split.next().unwrap(),
        main_split.next().unwrap(),
        main_split.next().unwrap(),
    );
    let mut rules_split = rules.splitn(2, '-');
    let (min, max) = (
        rules_split.next().unwrap().parse::<usize>().unwrap(),
        rules_split.next().unwrap().parse::<usize>().unwrap(),
    );
    let char = char_to_find.chars().nth(0).unwrap();
    (min, max, char, password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_demo() {
        assert_eq!(2, part1(&make_demo_input()))
    }

    #[test]
    fn part2_demo() {
        assert_eq!(1, part2(&make_demo_input()))
    }

    fn make_demo_input() -> Vec<String> {
        vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ]
    }
}
