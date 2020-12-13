use std::fs;

pub fn part1() -> usize {
    do_part1(&get_day_03_input(), 3, 1)
}

pub fn part2() -> usize {
    do_part2(&get_day_03_input())
}

pub fn do_part1(map: &[String], go_right: usize, go_down: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut trees: usize = 0;
    let max_lines = map.iter().count();
    let mut line_width: usize = 0;
    while y < max_lines {
        let line = map[y].clone();
        if 0 == line_width {
            line_width = line.chars().count();
        }
        if x >= line_width {
            x -= line_width;
        }
        let char = line
            .chars()
            .nth(x)
            .unwrap_or_else(|| panic!("Expected char at index: {} max: {}", x, line_width));
        if '#' == char {
            trees += 1;
        }
        y += go_down;
        x += go_right;
    }
    trees
}

fn do_part2(map: &[String]) -> usize {
    do_part1(&map, 1, 1)
        * do_part1(&map, 3, 1)
        * do_part1(&map, 5, 1)
        * do_part1(&map, 7, 1)
        * do_part1(&map, 1, 2)
}

fn get_day_03_input() -> Vec<String> {
    fs::read_to_string("./input/day_03.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_demo() {
        assert_eq!(7, do_part1(&get_demo_map(), 3, 1));
    }

    #[test]
    fn part2_demo() {
        assert_eq!(336, do_part2(&get_demo_map()));
    }

    fn get_demo_map() -> Vec<String> {
        vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ]
    }
}
