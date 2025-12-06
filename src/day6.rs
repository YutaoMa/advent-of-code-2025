pub fn run() {
    let input = include_str!("../data/day6_real.txt");
    println!("Day 6 Part 1: {}", part1(input));
    println!("Day 6 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let (ops_line, num_lines) = lines.split_last().expect("Input cannot be empty");

    let ops: Vec<char> = ops_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let nums: Vec<Vec<u64>> = num_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    (0..ops.len())
        .map(|col| {
            let col_vals = nums.iter().map(|row| row[col]);
            match ops[col] {
                '*' => col_vals.product(),
                '+' => col_vals.sum(),
                _ => 0,
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = grid.iter().map(|r| r.len()).max().unwrap_or(0);

    let mut grand_total = 0;
    let mut stack = Vec::new();

    for col in (0..width).rev() {
        let col_str: String = grid
            .iter()
            .filter_map(|row| row.get(col))
            .filter(|&&c| c != ' ')
            .collect();

        if col_str.is_empty() {
            stack.clear();
            continue;
        }

        if let Some(op) = col_str.chars().last().filter(|&c| c == '*' || c == '+') {
            let num = col_str[..col_str.len() - 1].parse::<u64>().unwrap();
            stack.push(num);
            
            grand_total += match op {
                '*' => stack.iter().product(),
                '+' => stack.iter().sum(),
                _ => 0,
            };
        } else {
            stack.push(col_str.parse().unwrap());
        }
    }
    grand_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../data/day6_example.txt");
        assert_eq!(part1(input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../data/day6_example.txt");
        assert_eq!(part2(input), 3263827);
    }
}
