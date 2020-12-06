pub fn part1(map: &Vec<String>, go_right: usize, go_down: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut trees: usize = 0;
    let max_lines = map.iter().count();
    let mut line_width: usize = 0;
    while y < max_lines {
        let line = map.iter().nth(y).unwrap();
        if 0 == line_width {
            line_width = line.chars().count();
        }
        if x >= line_width {
            x = x - line_width;
        }
        let char = line.chars().nth(x).expect(&format!(
            "Expected char at index: {} max: {}",
            x, line_width
        ));
        if '#' == char {
            trees += 1;
        }
        y += go_down;
        x += go_right;
    }
    trees
}

pub fn part2(_map: &Vec<String>) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_demo() {
        assert_eq!(7, part1(&get_demo_map(), 3, 1));
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
