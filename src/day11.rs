use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("../data/day11_real.txt");
    println!("Day 11 Part 1: {}", part1(input));
    println!("Day 11 Part 2: {}", part2(input));
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
    let adjacency_list = parse_input(input);
    let checkpoints: Vec<&str> = vec!["dac", "fft"];

    let checkpoint_idx: HashMap<&str, usize> = checkpoints
        .iter()
        .enumerate()
        .map(|(i, &cp)| (cp, i))
        .collect();

    let all_checkpoints_mask = (1u8 << checkpoints.len()) - 1;

    let mut memo: HashMap<(String, u8), u64> = HashMap::new();

    count_paths_memo(
        "svr",
        0,
        &adjacency_list,
        &checkpoint_idx,
        all_checkpoints_mask,
        &mut memo,
    )
}

fn count_paths_memo(
    node: &str,
    visited_mask: u8,
    adjacency_list: &HashMap<String, HashSet<String>>,
    checkpoint_idx: &HashMap<&str, usize>,
    all_checkpoints_mask: u8,
    memo: &mut HashMap<(String, u8), u64>,
) -> u64 {
    let current_mask = if let Some(&idx) = checkpoint_idx.get(node) {
        visited_mask | (1 << idx)
    } else {
        visited_mask
    };

    if node == "out" {
        return if current_mask == all_checkpoints_mask {
            1
        } else {
            0
        };
    }

    let key = (node.to_string(), current_mask);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut total = 0;
    if let Some(neighbors) = adjacency_list.get(node) {
        for neighbor in neighbors {
            total += count_paths_memo(
                neighbor,
                current_mask,
                adjacency_list,
                checkpoint_idx,
                all_checkpoints_mask,
                memo,
            );
        }
    }

    memo.insert(key, total);
    total
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

    #[test]
    fn test_part2() {
        let input = include_str!("../data/day11_example_b.txt");
        assert_eq!(part2(input), 2);
    }
}
