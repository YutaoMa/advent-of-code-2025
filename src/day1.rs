fn day1_a(input: &str) -> u32 {
    let mut pos = 50;
    let mut zero_count = 0;

    for rotation in input.lines() {
        let rotation = parse_rotation(rotation);
        pos += rotation;
        pos %= 100;
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
}
