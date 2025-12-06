pub fn run() {
    let input = include_str!("../data/day6_real.txt");
    println!("Day 6 Part 1: {}", part1(input));
    println!("Day 6 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let (nums, operations) = parse_input(input);
    let mut total = 0;
    for col in 0..nums[0].len() {
        let mut result = if operations[col] == '*' { 1 } else { 0 };
        for row in 0..nums.len() {
            if operations[col] == '*' {
                result *= nums[row][col];
            } else {
                result += nums[row][col];
            }
        }
        total += result;
    }
    return total;
}

fn part2(input: &str) -> u64 {
    0
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let nums = lines[..lines.len() - 1].iter().map(|line| line.split_whitespace().map(|num| num.parse().unwrap()).collect()).collect();
    let operations = lines[lines.len() - 1].split_whitespace().map(|op| op.parse().unwrap()).collect();
    (nums, operations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../data/day6_example.txt");
        assert_eq!(part1(input), 4277556);
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../data/day6_example.txt");
        let (nums, operations) = parse_input(input);
        assert_eq!(nums, vec![vec![123, 328, 51, 64], vec![45, 64, 387, 23], vec![6, 98, 215, 314]]);
        assert_eq!(operations, vec!['*', '+', '*', '+']);
    }
}