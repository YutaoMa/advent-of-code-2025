pub fn run() {
    let input = include_str!("../data/day3_real.txt");
    println!("Day 3 Part 1: {}", part1(input));
    println!("Day 3 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    input.lines().map(|num| largest_subsequence(num, 2)).sum()
}

fn part2(input: &str) -> u64 {
    input.lines().map(|num| largest_subsequence(num, 12)).sum()
}


fn largest_subsequence(num: &str, k: usize) -> u64 {
    let digits: Vec<u8> = num.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u8).collect();
    let len = digits.len();
    if len < k {
        return 0;
    }

    // Monotonic stack!
    let mut stack = Vec::with_capacity(k);
    for (i, &digit) in digits.iter().enumerate() {
        let remaining = len - i;

        while let Some(&top) = stack.last() {
            if digit > top && stack.len() + remaining > k {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.len() < k {
            stack.push(digit);
        }
    }

    stack.into_iter().fold(0, |acc, digit| acc * 10 + digit as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_subsequence() {
        assert_eq!(largest_subsequence("987654321111111", 2), 98);
        assert_eq!(largest_subsequence("811111111111119", 2), 89);
        assert_eq!(largest_subsequence("234234234234278", 2), 78);
        assert_eq!(largest_subsequence("818181911112111", 2), 92);

        assert_eq!(largest_subsequence("987654321111111", 12), 987654321111);
        assert_eq!(largest_subsequence("811111111111119", 12), 811111111119);
        assert_eq!(largest_subsequence("234234234234278", 12), 434234234278);
        assert_eq!(largest_subsequence("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_part1_example() {
        let input = include_str!("../data/day3_example.txt");
        assert_eq!(part1(input), 357);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../data/day3_example.txt");
        assert_eq!(part2(input), 3121910778619);
    }
}