use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashSet<String>>;

pub fn run() {
    let input = include_str!("../data/day11_real.txt");
    println!("Day 11 Part 1: {}", part1(input));
    println!("Day 11 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    count_paths("you", &parse_input(input), &[])
}

fn part2(input: &str) -> u64 {
    count_paths("svr", &parse_input(input), &["dac", "fft"])
}

fn count_paths(start: &str, graph: &Graph, checkpoints: &[&str]) -> u64 {
    let checkpoint_bits: HashMap<&str, u8> = checkpoints
        .iter()
        .enumerate()
        .map(|(i, &cp)| (cp, 1 << i))
        .collect();
    let required = (1u8 << checkpoints.len()) - 1;
    let mut memo = HashMap::new();

    count_paths_inner(start, 0, graph, &checkpoint_bits, required, &mut memo)
}

fn count_paths_inner(
    node: &str,
    mask: u8,
    graph: &Graph,
    checkpoint_bits: &HashMap<&str, u8>,
    required: u8,
    memo: &mut HashMap<(String, u8), u64>,
) -> u64 {
    let mask = mask | checkpoint_bits.get(node).copied().unwrap_or(0);

    if node == "out" {
        return u64::from(mask == required);
    }

    let key = (node.to_string(), mask);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let total = graph
        .get(node)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|n| count_paths_inner(n, mask, graph, checkpoint_bits, required, memo))
                .sum()
        })
        .unwrap_or(0);

    memo.insert(key, total);
    total
}

fn parse_input(input: &str) -> Graph {
    input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(from, to)| {
            (
                from.to_string(),
                to.split_whitespace().map(String::from).collect(),
            )
        })
        .collect()
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
            HashSet::from_iter(["bbb".to_string(), "ccc".to_string()])
        );
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../data/day11_example_b.txt");
        assert_eq!(part2(input), 2);
    }
}
