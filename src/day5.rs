pub fn run() {
    let input = include_str!("../data/day5_real.txt");
    println!("Day 5 Part 1: {}", part1(input));
    println!("Day 5 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let (ranges, numbers) = parse_input(input);
    let merged = merge_ranges(ranges);
    numbers
        .iter()
        .filter(|&&n| is_in_ranges(n, &merged))
        .count() as u64
}

fn part2(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);
    merge_ranges(ranges)
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges_str, numbers_str) = input.split_once("\n\n").expect("Invalid input format");

    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').expect("Invalid range format");
            (
                start.parse().expect("Invalid start"),
                end.parse().expect("Invalid end"),
            )
        })
        .collect();

    let numbers = numbers_str
        .lines()
        .map(|line| line.parse().expect("Invalid number"))
        .collect();

    (ranges, numbers)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_unstable_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1.saturating_add(1) {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }
    merged
}

fn is_in_ranges(number: u64, ranges: &[(u64, u64)]) -> bool {
    // Binary search!
    let idx = ranges.partition_point(|&(start, _)| start <= number);
    
    if idx == 0 {
        false
    } else {
        let (_, end) = ranges[idx - 1];
        number <= end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/day5_example.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 14);
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
        assert!(is_in_ranges(11, &ranges));
        assert!(!is_in_ranges(32, &ranges));
    }
}
