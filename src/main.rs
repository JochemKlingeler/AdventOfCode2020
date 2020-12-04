use day_01;
use std::fs;

fn main() {
    println!("Day 1, part 1 solution is: {}", exec_day_01_part_1());
    println!("Day 1, part 2 solution is: {}", exec_day_01_part_2());
}

fn exec_day_01_part_1() -> u32 {
    day_01::part1(&get_day_01_input()).expect("Expected a working solution!")
}

fn exec_day_01_part_2() -> u32 {
    day_01::part2(&get_day_01_input()).expect("Expected a working solution!")
}

fn get_day_01_input() -> Vec<u32> {
    let values =
        fs::read_to_string("./input/day_01.txt").expect("Something went wrong reading the file");
    values
        .lines()
        .map(|line| line.parse::<u32>().expect("All values should be u32s"))
        .collect()
}
