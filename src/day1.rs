fn day1_a(input: &str) -> u32 {
    let mut pos = 50;
    let mut zero_count = 0;

    for rotation in input.lines() {
        let rotation = parse_rotation(rotation);
        pos += rotation;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

// Lnum is -num, Rnum is +num
fn parse_rotation(rotation: &str) -> i32 {
    let num = rotation[1..].parse::<i32>().unwrap();
    match rotation.chars().nth(0).unwrap() {
        'L' => -num,
        'R' => num,
        _ => panic!("Invalid rotation: {}", rotation),
    }
}

fn day1_b(input: &str) -> u32 {
    let mut pos = 50;
    let mut crossings = 0;

    for rotation in input.lines() {
        let rotation = parse_rotation(rotation);
        let crossings_change = count_zero_crossings(pos, rotation);
        crossings += crossings_change;
        pos += rotation;
        pos = pos.rem_euclid(100);
    }

    crossings
}

// We count landing at 0 as a crossing
// But not when starting at 0
fn count_zero_crossings(pos: i32, rotation: i32) -> u32 {
    let offset = if rotation >= 0 { 0 } else { 1 };
    let end = (pos + rotation - offset).div_euclid(100);
    let start = (pos - offset).div_euclid(100);
    (end - start).abs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_a_example() {
        let input = include_str!("../data/day1_a_example.txt");
        assert_eq!(day1_a(input), 3);
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
    fn test_day1_a_real() {
        let input = include_str!("../data/day1_a_real.txt");
        println!("{}", day1_a(input));
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
    fn test_day1_b_example() {
        let input = include_str!("../data/day1_a_example.txt");
        assert_eq!(day1_b(input), 6);
    }

    #[test]
    fn test_day1_b_real() {
        let input = include_str!("../data/day1_a_real.txt");
        println!("{}", day1_b(input));
    }
}
