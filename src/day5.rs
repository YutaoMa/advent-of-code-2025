pub fn run() {
    let input = include_str!("../data/day5_real.txt");
    println!("Day 5 Part 1: {}", part1(input));
    println!("Day 5 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let (ranges, numbers) = parse_input(input);
    let merged_ranges = merge_ranges(ranges);
    numbers.iter().filter(|&number| is_in_ranges(*number, &merged_ranges)).count() as u64
}

fn part2(input: &str) -> u64 {
    let (ranges, numbers) = parse_input(input);
    let merged_ranges = merge_ranges(ranges);
    total_range(&merged_ranges)
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, numbers) = input.split_once("\n\n").unwrap();
    (parse_ranges(ranges), parse_numbers(numbers))
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect()
}

fn parse_numbers(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by_key(|(start, _)| *start);
    let mut merged = Vec::new();
    for (start, end) in ranges {
        if let Some((last_start, last_end)) = merged.last_mut() {
            if *last_end >= start - 1 {
                *last_end = end.max(*last_end);
                continue;
            }
        }
        merged.push((start, end));
    }
    merged
}

fn is_in_ranges(number: u64, ranges: &Vec<(u64, u64)>) -> bool {
    ranges.iter().any(|(start, end)| number >= *start && number <= *end)
}

fn total_range(ranges: &Vec<(u64, u64)>) -> u64 {
    ranges.iter().fold(0, |acc, (start, end)| acc + (end - start + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../data/day5_example.txt");
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../data/day5_example.txt");
        let (ranges, numbers) = parse_input(input);
        assert_eq!(ranges, vec![(3, 5), (10, 14), (16, 20), (12, 18)]);
        assert_eq!(numbers, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let merged = merge_ranges(ranges);
        assert_eq!(merged, vec![(3, 5), (10, 20)]);
    }

    #[test]
    fn test_is_in_ranges() {
        let ranges = vec![(3, 5), (10, 20)];
        assert!(!is_in_ranges(1, &ranges));
        assert!(is_in_ranges(5, &ranges));
        assert!(!is_in_ranges(8, &ranges));
        assert!(is_in_ranges(11, &ranges));
        assert!(is_in_ranges(17, &ranges));
        assert!(!is_in_ranges(32, &ranges));
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../data/day5_example.txt");
        assert_eq!(part2(input), 14);
    }
}