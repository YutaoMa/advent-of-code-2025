pub fn run() {
    let input = include_str!("../data/day7_real.txt");
    println!("Day 7 Part 1: {}", part1(input));
    println!("Day 7 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    solve(input, true)
}

fn part2(input: &str) -> u64 {
    solve(input, false)
}

fn solve(input: &str, part1: bool) -> u64 {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    if grid.is_empty() {
        return 0;
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut beams = vec![0u64; cols];
    let mut next_beams = vec![0u64; cols];
    let mut hit_count = 0;

    for r in 1..rows {
        next_beams.fill(0);
        let prev_row = grid[r - 1];
        let curr_row = grid[r];

        for c in 0..cols {
            let is_source = prev_row[c] == b'S';
            let incoming = beams[c];

            if is_source {
                next_beams[c] = 1;
            } else if incoming > 0 {
                match curr_row[c] {
                    b'^' => {
                        if part1 {
                            hit_count += 1;
                            if c > 0 {
                                next_beams[c - 1] = 1;
                            }
                            if c < cols - 1 {
                                next_beams[c + 1] = 1;
                            }
                        } else {
                            if c > 0 {
                                next_beams[c - 1] += incoming;
                            }
                            if c < cols - 1 {
                                next_beams[c + 1] += incoming;
                            }
                        }
                    }
                    _ => {
                        if part1 {
                            next_beams[c] = 1;
                        } else {
                            next_beams[c] += incoming;
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut beams, &mut next_beams);
    }

    if part1 {
        hit_count
    } else {
        beams.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../data/day7_example.txt");
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../data/day7_example.txt");
        assert_eq!(part2(input), 40);
    }
}
