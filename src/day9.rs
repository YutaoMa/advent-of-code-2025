pub fn run() {
    let input = include_str!("../data/day9_real.txt");
    println!("Day 9 Part 1: {}", part1(input));
    // println!("Day 9 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let coords = parse_coords(input);
    find_largest_area(&coords)
}

fn part2(input: &str) -> u64 {
    0
}

fn parse_coords(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn find_largest_area(coords: &[(i64, i64)]) -> u64 {
    let mut max_area = 0;
    for coord in coords {
        for other in coords {
            let area = ((coord.0 - other.0).abs() + 1) * ((coord.1 - other.1).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../data/day9_example.txt");
        assert_eq!(part1(input), 50);
    }
}
