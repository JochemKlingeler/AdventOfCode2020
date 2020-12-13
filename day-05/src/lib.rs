pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|f| get_id(f))
        .max()
        .expect("Expected a max value")
}

pub fn part2(_input: &String) -> usize {
    unimplemented!("Not implemented");
}

fn get_id(input: &str) -> usize {
    let row = get_row(input);
    let seat = get_seat(input);
    let id = row * 8 + seat;
    id
}

fn get_row(input: &str) -> usize {
    let mut range: Vec<usize> = (0..128).collect();
    for char in input[..7].chars() {
        match char {
            'B' => {
                range = range[(range.len() / 2)..].to_vec();
            }
            'F' => {
                range = range[..(range.len() / 2)].to_vec();
            }
            _ => panic!("Expected either char 'B' or 'F' but got: {}", char),
        }
    }
    range
        .first()
        .expect("Expected range to not be empty")
        .clone()
}

fn get_seat(input: &str) -> usize {
    let mut range: Vec<usize> = (0..8).collect();
    for char in input[7..].chars() {
        match char {
            'R' => {
                range = range[(range.len() / 2)..].to_vec();
            }
            'L' => {
                range = range[..(range.len() / 2)].to_vec();
            }
            _ => panic!("Expected either char 'L' or 'R' but got: {}", char),
        }
    }
    range
        .first()
        .expect("Expected seat range to not be empty")
        .clone()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_row_1() {
        assert_eq!(44, get_row("FBFBBFFRLR"))
    }
    #[test]
    fn test_get_row_2() {
        assert_eq!(70, get_row("BFFFBBFRRR"))
    }
    #[test]
    fn test_get_row_3() {
        assert_eq!(14, get_row("FFFBBBFRRR"))
    }
    #[test]
    fn test_get_row_4() {
        assert_eq!(102, get_row("BBFFBBFRLL"))
    }
    #[test]
    fn test_get_seat_1() {
        assert_eq!(5, get_seat("FBFBBFFRLR"))
    }
    #[test]
    fn test_get_seat_2() {
        assert_eq!(7, get_seat("BFFFBBFRRR"))
    }
    #[test]
    fn test_get_seat_3() {
        assert_eq!(7, get_seat("FFFBBBFRRR"))
    }
    #[test]
    fn test_get_seat_4() {
        assert_eq!(4, get_seat("BBFFBBFRLL"))
    }
    #[test]
    fn test_get_id_1() {
        assert_eq!(357, get_id("FBFBBFFRLR"))
    }
    #[test]
    fn test_get_id_2() {
        assert_eq!(567, get_id("BFFFBBFRRR"))
    }
    #[test]
    fn test_get_id_3() {
        assert_eq!(119, get_id("FFFBBBFRRR"))
    }
    #[test]
    fn test_get_id_4() {
        assert_eq!(820, get_id("BBFFBBFRLL"))
    }
}
