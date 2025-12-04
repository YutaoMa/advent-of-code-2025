pub fn run() {
    let input = include_str!("../data/day3_real.txt");
    println!("Day 3 Part 1: {}", part1(input));
    println!("Day 3 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    input.lines().map(largest_two_digits).sum()
}

fn part2(input: &str) -> u64 {
    input.lines().map(largest_twelve_digits).sum()
}

fn largest_two_digits(num: &str) -> u64 {
    let digits: Vec<u8> = num.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u8).collect();
    let mut max_num = 0;
    let mut max_right = 0u8;
    for i in (0..digits.len()).rev() {
        let d = digits[i];
        if max_right > 0 {
            let candidate = d as u64 * 10 + max_right as u64;
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

fn largest_twelve_digits(num: &str) -> u64 {
    let digits: Vec<u8> = num.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u8).collect();
    let len = digits.len();
    if len < 12 {
        return 0;
    }

    let mut max_num = 0u64;
    let mut last_pos = 0;

    for i in 0..12 {
        let remaining_needed = 11 - i;
        let search_end = len - 1 - remaining_needed;

        let mut max_digit = 0;
        let mut chosen_index = last_pos;

        for pos in last_pos..=search_end {
            let d = digits[pos];
            if d > max_digit {
                max_digit = d;
                chosen_index = pos;

                // No need to check further if we already have a 9
                if max_digit == 9 {
                    break;
                }
            }
        }

        max_num = max_num * 10 + max_digit as u64;
        last_pos = chosen_index + 1;
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

    #[test]
    fn test_largest_twelve_digits() {
        assert_eq!(largest_twelve_digits("987654321111111"), 987654321111);
        assert_eq!(largest_twelve_digits("811111111111119"), 811111111119);
        assert_eq!(largest_twelve_digits("234234234234278"), 434234234278);
        assert_eq!(largest_twelve_digits("818181911112111"), 888911112111);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../data/day3_example.txt");
        assert_eq!(part2(input), 3121910778619);
    }
}