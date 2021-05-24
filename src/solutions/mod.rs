mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
pub fn exec_day(day: &u8) {
    match day {
        1 => println!(
            "Day 1, part 1: {}, part 2: {}",
            day_01::part1(),
            day_01::part2()
        ),
        2 => println!(
            "Day 2, part 1: {}, part 2: {}",
            day_02::part1(),
            day_02::part2()
        ),
        3 => println!(
            "Day 3, part 1: {}, part 2: {}",
            day_03::part1(),
            day_03::part2()
        ),
        4 => println!(
            "Day 4, part 1: {}, part 2: {}",
            day_04::part1(),
            day_04::part2()
        ),
        5 => println!(
            "Day 5, part 1: {}, part 2: {}",
            day_05::part1(),
            day_05::part2()
        ),
        6 => println!(
            "Day 6, part 1: {}, part 2: {}",
            day_06::part1(),
            day_06::part2()
        ),
        7 => println!(
            "Day 7, part 1: {}, part 2: {}",
            day_07::part1(),
            day_07::part2()
        ),
        _ => println!("Day {} is not implemented yet!", day),
    }
}
