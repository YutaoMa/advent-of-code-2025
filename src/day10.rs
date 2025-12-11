use std::collections::{HashSet, VecDeque};

pub fn run() {
    let input = include_str!("../data/day10_real.txt");
    println!("Day 10 Part 1: {}", part1(input));
    // println!("Day 10 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let lines = parse_input(input);
    lines
        .into_iter()
        .map(|(goal, instructions)| min_operations(&goal, &instructions))
        .sum()
}

fn part2(input: &str) -> u64 {
    0
}

fn min_operations(goal: &[bool], instructions: &[Vec<usize>]) -> u64 {
    let n = goal.len();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let start = vec![false; n];
    queue.push_back((start.clone(), 0));
    visited.insert(start);

    while let Some((state, steps)) = queue.pop_front() {
        if state == goal {
            return steps;
        }
        for instr in instructions {
            let mut next = state.clone();
            for &idx in instr {
                if idx < n {
                    next[idx] ^= true;
                }
            }
            if visited.insert(next.clone()) {
                queue.push_back((next, steps + 1));
            }
        }
    }

    u64::MAX
}

fn parse_input(input: &str) -> Vec<(Vec<bool>, Vec<Vec<usize>>)> {
    input
        .lines()
        .map(|line| {
            let mut sections = line.split_whitespace();
            let goal = sections.next().unwrap();
            let instruction_sections: Vec<&str> = sections
                .by_ref()
                .take_while(|s| !s.starts_with('{'))
                .collect();
            (
                goal[1..goal.len() - 1]
                    .chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Unexpected char in goal"),
                    })
                    .collect::<Vec<bool>>(),
                instruction_sections
                    .iter()
                    .map(|s| {
                        let inner = s.trim_start_matches('(').trim_end_matches(')');
                        inner
                            .split(',')
                            .map(|num| num.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<Vec<usize>>>(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../data/day10_example.txt");
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../data/day10_example.txt");
        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].0, vec![false, true, true, false]);
        assert_eq!(
            parsed[0].1,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
        assert_eq!(parsed[1].0, vec![false, false, false, true, false]);
        assert_eq!(
            parsed[1].1,
            vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4]
            ]
        );
    }
}
