use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("../data/day11_real.txt");
    println!("Day 11 Part 1: {}", part1(input));
    // println!("Day 11 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let adjacency_list = parse_input(input);

    dfs("you", &adjacency_list, &mut HashSet::new())
}

fn dfs(
    node: &str,
    adjacency_list: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
) -> u64 {
    if node == "out" {
        return 1;
    }
    visited.insert(node.to_string());
    let mut count = 0;
    if let Some(neighbors) = adjacency_list.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor.as_str()) {
                count += dfs(neighbor, adjacency_list, visited);
            }
        }
    }
    visited.remove(node);
    count
}

fn part2(input: &str) -> u64 {
    0
}

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once(": ").unwrap();
        map.insert(
            from.to_string(),
            to.split_whitespace()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
        );
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../data/day11_example.txt");
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../data/day11_example.txt");
        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 10);
        assert_eq!(
            parsed["you"],
            HashSet::from_iter(vec!["bbb".to_string(), "ccc".to_string()])
        );
    }
}
