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
    let grid = parse_grid(input);
    let mut grand_total = 0;
    let mut nums = Vec::new();
    for col in (0..grid[0].len()).rev() {
        let mut curr_str = String::new();
        for row in 0..grid.len() {
            if grid[row][col] != ' ' && grid[row][col] != '\n' {
                curr_str.push(grid[row][col]);
            }
        }
        if curr_str.is_empty() {
            nums.clear();
            continue;
        }
        if curr_str.ends_with('*') {
            nums.push(curr_str[..curr_str.len() - 1].parse().unwrap());
            grand_total += nums.iter().product::<u64>();
        } else if curr_str.ends_with('+') {
            nums.push(curr_str[..curr_str.len() - 1].parse().unwrap());
            grand_total += nums.iter().sum::<u64>();
        } else {
            nums.push(curr_str.parse().unwrap());
        }
    }
    grand_total
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let nums = lines[..lines.len() - 1].iter().map(|line| line.split_whitespace().map(|num| num.parse().unwrap()).collect()).collect();
    let operations = lines[lines.len() - 1].split_whitespace().map(|op| op.parse().unwrap()).collect();
    (nums, operations)
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
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
    fn test_part2_example() {
        let input = include_str!("../data/day6_example.txt");
        assert_eq!(part2(input), 3263827);
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../data/day6_example.txt");
        let (nums, operations) = parse_input(input);
        assert_eq!(nums, vec![vec![123, 328, 51, 64], vec![45, 64, 387, 23], vec![6, 98, 215, 314]]);
        assert_eq!(operations, vec!['*', '+', '*', '+']);
    }
}