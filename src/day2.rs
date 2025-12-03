pub fn run() {
    let input = include_str!("../data/day2_real.txt");
    println!("Day 2 Part 1: {}", part1(input));
    // println!("Day 2 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    parse_ranges(input)
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|&num| is_invalid(num))
        .map(|num| num)
        .sum()
}

fn part2(input: &str) -> u64 {
    todo!();
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|range| {
            let mut nums = range.split('-').map(|num| num.parse::<u64>().unwrap());
            let start = nums.next().unwrap();
            let end = nums.next().unwrap();
            (start, end)
        })
        .collect()
}

// Invalid if num string repr is made up of two identical parts
// e.g. 11, 222, 3333, 44444444
// No leading zeros according to the spec
fn is_invalid(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let mid = len / 2;
    &s[..mid] == &s[mid..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../data/day2_example.txt");
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_is_invalid() {
        assert_eq!(is_invalid(55), true);
        assert_eq!(is_invalid(6464), true);
        assert_eq!(is_invalid(123123), true);
        assert_eq!(is_invalid(101), false);
    }
}
