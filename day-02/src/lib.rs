use std::convert::TryInto;

pub fn part1(input: &Vec<String>) -> usize {
    input.iter().filter(|i| is_valid(&i)).count()
}

fn is_valid(input: &String) -> bool {
    let mut main_split = input.splitn(3, ' ');
    let (rules, char_to_find, password) = (
        main_split.next().unwrap(),
        main_split.next().unwrap(),
        main_split.next().unwrap(),
    );
    let mut rules_split = rules.splitn(2, '-');
    let (min, max) = (
        rules_split.next().unwrap().parse::<u8>().unwrap(),
        rules_split.next().unwrap().parse::<u8>().unwrap(),
    );
    let char = char_to_find.chars().nth(0).unwrap();

    let char_count: u8 = password
        .chars()
        .filter(|c| c == &char)
        .count()
        .try_into()
        .unwrap();
    min <= char_count && char_count <= max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_demo() {
        assert_eq!(2, part1(&make_demo_input()))
    }

    fn make_demo_input() -> Vec<String> {
        vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ]
    }
}
