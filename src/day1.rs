const START_POS: i32 = 50;
const MODULO: i32 = 100;

pub fn run() {
    let input = include_str!("../data/day1_real.txt");
    println!("Day 1 Part 1: {}", part1(input));
    println!("Day 1 Part 2: {}", part2(input));
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_rotation)
        .fold((START_POS, 0), |(pos, count), rot| {
            let next_pos = (pos + rot).rem_euclid(MODULO);
            let next_count = if next_pos == 0 { count + 1 } else { count };
            (next_pos, next_count)
        })
        .1
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_rotation)
        .fold((START_POS, 0), |(pos, crossings), rot| {
            let change = count_zero_crossings(pos, rot);
            let next_pos = (pos + rot).rem_euclid(MODULO);
            (next_pos, crossings + change)
        })
        .1
}

// Lnum is -num, Rnum is +num
fn parse_rotation(rotation: &str) -> i32 {
    if let Some(val) = rotation.strip_prefix('L') {
        -val.parse::<i32>().expect("valid integer after L")
    } else if let Some(val) = rotation.strip_prefix('R') {
        val.parse::<i32>().expect("valid integer after R")
    } else {
        panic!("Invalid rotation format: {}", rotation);
    }
}

// We count landing at 0 as a crossing
// But not when starting at 0
fn count_zero_crossings(pos: i32, rotation: i32) -> u32 {
    let offset = if rotation >= 0 { 0 } else { 1 };
    let end = (pos + rotation - offset).div_euclid(MODULO);
    let start = (pos - offset).div_euclid(MODULO);
    (end - start).abs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../data/day1_example.txt");
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_parse_rotation() {
        assert_eq!(parse_rotation("L68"), -68);
        assert_eq!(parse_rotation("R48"), 48);
        assert_eq!(parse_rotation("L5"), -5);
        assert_eq!(parse_rotation("R60"), 60);
        assert_eq!(parse_rotation("L55"), -55);
        assert_eq!(parse_rotation("L1"), -1);
    }

    #[test]
    fn test_count_zero_crossings() {
        assert_eq!(count_zero_crossings(50, -68), 1);
        assert_eq!(count_zero_crossings(52, 48), 1);
        assert_eq!(count_zero_crossings(95, 60), 1);
        assert_eq!(count_zero_crossings(55, -55), 1);
        assert_eq!(count_zero_crossings(99, -99), 1);
        assert_eq!(count_zero_crossings(14, -82), 1);
        assert_eq!(count_zero_crossings(50, 1000), 10);
        assert_eq!(count_zero_crossings(82, -30), 0);
        assert_eq!(count_zero_crossings(52, 48), 1);
        assert_eq!(count_zero_crossings(0, 10), 0);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../data/day1_example.txt");
        assert_eq!(part2(input), 6);
    }
}
