use std::fs;

fn main() {
    // println!("Day 1, part 1 solution is: {}", exec_day_01_part_1());
    // println!("Day 1, part 2 solution is: {}", exec_day_01_part_2());
    // println!("Day 2, part 1 solution is: {}", exec_day_02_part_1());
    // println!("Day 2, part 2 solution is: {}", exec_day_02_part_2());
    // println!("Day 3, part 1 solution is: {}", exec_day_03_part_1());
    // println!("Day 3, part 2 solution is: {}", exec_day_03_part_2());
    println!("Day 4, part 1 solution is: {}", exec_day_04_part_1());
}
fn exec_day_04_part_1() -> usize {
    day_04::part1(&get_day_04_input())
}

fn get_day_04_input() -> String {
    fs::read_to_string("./input/day_04.txt").expect("Something went wrong reading the file")
}

#[allow(dead_code)]
fn exec_day_03_part_1() -> usize {
    day_03::part1(&get_day_03_input(), 3, 1)
}

#[allow(dead_code)]
fn exec_day_03_part_2() -> usize {
    day_03::part2(&get_day_03_input())
}

#[allow(dead_code)]
fn get_day_03_input() -> Vec<String> {
    fs::read_to_string("./input/day_03.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect()
}

#[allow(dead_code)]
fn exec_day_02_part_1() -> usize {
    day_02::part1(&get_day_02_input())
}

#[allow(dead_code)]
fn exec_day_02_part_2() -> usize {
    day_02::part2(&get_day_02_input())
}

#[allow(dead_code)]
fn get_day_02_input() -> Vec<String> {
    fs::read_to_string("./input/day_02.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect()
}

#[allow(dead_code)]
fn exec_day_01_part_1() -> u32 {
    day_01::part1(&get_day_01_input()).expect("Expected a working solution!")
}

#[allow(dead_code)]
fn exec_day_01_part_2() -> u32 {
    day_01::part2(&get_day_01_input()).expect("Expected a working solution!")
}

#[allow(dead_code)]
fn get_day_01_input() -> Vec<u32> {
    fs::read_to_string("./input/day_01.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse::<u32>().expect("All values should be u32s"))
        .collect()
}
