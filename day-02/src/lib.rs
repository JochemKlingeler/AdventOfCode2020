use std::convert::TryInto;

pub fn part1(input: &Vec<String>) -> u16 {
    input
        .iter()
        .filter(|i| is_valid(i))
        .count()
        .try_into()
        .unwrap()
}

fn is_valid(input: &String) -> bool {
    // If only Rust had regexes, grr, there has to be an easier way
    let mut split = input.splitn(3, ' ');
    let amount_rules = split.next().unwrap();
    let mut amount_rules_split = amount_rules.split('-');
    let min = amount_rules_split.next().unwrap().parse::<u8>().unwrap();
    let max = amount_rules_split.next().unwrap().parse::<u8>().unwrap();
    // TODO: Future me, this char is empty string? Fix me!
    let char = split.next().unwrap().chars().nth(0).unwrap();
    let password = split.next().unwrap();

    let char_count = password
        .chars()
        .filter(|c| c == &char)
        .count()
        .try_into()
        .unwrap();
    min >= char_count || char_count <= max
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
