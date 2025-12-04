pub fn run() {
    let input = include_str!("../data/day3_real.txt");
    println!("Day 3 Part 1: {}", part1(input));
    // println!("Day 3 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.lines().map(largest_two_digits).sum()
}

fn part2(input: &str) -> u32 {
    0
}

fn largest_two_digits(num: &str) -> u32 {
    let digits: Vec<u8> = num.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u8).collect();
    let mut max_num = 0;
    let mut max_right = 0u8;
    for i in (0..digits.len()).rev() {
        let d = digits[i];
        if max_right > 0 {
            let candidate = d as u32 * 10 + max_right as u32;
            if candidate > max_num {
                max_num = candidate;
            }
        }
        if d > max_right {
            max_right = d;
        }
    }
    max_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_two_digits() {
        assert_eq!(largest_two_digits("987654321111111"), 98);
        assert_eq!(largest_two_digits("811111111111119"), 89);
        assert_eq!(largest_two_digits("234234234234278"), 78);
        assert_eq!(largest_two_digits("818181911112111"), 92);
    }

    #[test]
    fn test_part1_example() {
        let input = include_str!("../data/day3_example.txt");
        assert_eq!(part1(input), 357);
    }
}