use std::fs;

pub fn part1() -> usize {
    do_part1(&get_day_07_input())
}

fn do_part1(_input: &str) -> usize {
    0
}

pub fn part2() -> usize {
    do_part2(&get_day_07_input())
}

fn do_part2(_input: &str) -> usize {
    0
}

fn get_day_07_input() -> String {
    fs::read_to_string("./input/day_07.txt").expect("Something went wrong reading the file")
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn part1_demo() {
        assert_eq!(do_part1(self::DEMO), 4);
    }
    #[test]
    fn part2_demo() {
        assert_eq!(do_part2(self::DEMO), 1);
    }
}
