use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

pub fn run() {
    let input = include_str!("../data/day10_real.txt");
    println!("Day 10 Part 1: {}", part1(input));
    println!("Day 10 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|line| min_operations(&line.goal, &line.instructions))
        .sum()
}

fn min_operations(goal: &[bool], instructions: &[Vec<usize>]) -> u64 {
    let n = goal.len();
    let start = vec![false; n];

    if start == goal {
        return 0;
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(start.clone());
    queue.push_back((start, 0u64));

    while let Some((state, steps)) = queue.pop_front() {
        for instr in instructions {
            let mut next = state.clone();
            for &idx in instr.iter().filter(|&&i| i < n) {
                next[idx] ^= true;
            }
            if next == goal {
                return steps + 1;
            }
            if visited.insert(next.clone()) {
                queue.push_back((next, steps + 1));
            }
        }
    }

    u64::MAX
}

fn part2(input: &str) -> u64 {
    let lines = parse_input(input);
    let config = Part2Config::from_env(lines.len());

    let compute_one = |i: usize| -> u64 {
        let line = &lines[i];
        let line_start = Instant::now();
        eprintln!(
            "Processing line {}/{} (joltage: {:?})",
            i + 1,
            config.total,
            line.joltage
        );

        let result = if config.per_line_progress {
            let prefix = format!("line {}/{}", i + 1, config.total);
            min_counter_operations_with_progress(&prefix, &line.joltage, &line.instructions)
        } else {
            min_counter_operations(&line.joltage, &line.instructions)
        };

        eprintln!(
            "  -> result: {} (elapsed: {:?})",
            result,
            line_start.elapsed()
        );

        assert_ne!(result, u64::MAX, "No solution found for line {}", i + 1);
        result
    };

    let sum = if config.threads == 1 {
        (0..config.total).map(|i| compute_one(i) as u128).sum()
    } else {
        run_parallel(config.threads, config.total, compute_one)
    };

    u64::try_from(sum).unwrap_or_else(|_| panic!("part2 sum overflowed u64 (sum={})", sum))
}

struct Part2Config {
    total: usize,
    threads: usize,
    per_line_progress: bool,
}

impl Part2Config {
    fn from_env(line_count: usize) -> Self {
        let progress_enabled = std::env::var("AOC_DAY10_PROGRESS")
            .ok()
            .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));

        let limit_lines = std::env::var("AOC_DAY10_LIMIT_LINES")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&v| v > 0);

        let total = limit_lines.unwrap_or(line_count).min(line_count);

        let threads = std::env::var("AOC_DAY10_THREADS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&t| t > 0)
            .unwrap_or_else(|| {
                std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(1)
            });

        let per_line_progress = progress_enabled && threads == 1;
        if progress_enabled && threads > 1 {
            eprintln!(
                "AOC_DAY10_PROGRESS is enabled, but AOC_DAY10_THREADS>1: \
                 disabling per-line progress logs. Set AOC_DAY10_THREADS=1 to debug."
            );
        }

        Self {
            total,
            threads,
            per_line_progress,
        }
    }
}

fn run_parallel<F>(threads: usize, total: usize, compute: F) -> u128
where
    F: Fn(usize) -> u64 + Sync,
{
    eprintln!(
        "Running Day 10 Part 2 with {} threads ({} lines)",
        threads, total
    );
    let next = AtomicUsize::new(0);

    std::thread::scope(|s| {
        (0..threads)
            .map(|_| {
                s.spawn(|| {
                    let mut local_sum: u128 = 0;
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= total {
                            break;
                        }
                        local_sum += compute(i) as u128;
                    }
                    local_sum
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().expect("worker thread panicked"))
            .sum()
    })
}

fn min_counter_operations_with_progress(
    prefix: &str,
    joltage: &[u32],
    instructions: &[Vec<usize>],
) -> u64 {
    CounterSolver::new(joltage, instructions).solve(Some(prefix))
}

fn min_counter_operations(joltage: &[u32], instructions: &[Vec<usize>]) -> u64 {
    CounterSolver::new(joltage, instructions).solve(None)
}

struct CounterSolver {
    masks: Vec<u16>,
    sizes: Vec<u32>,
    initial_rem: Vec<u16>,
    sum_target: u32,
    use_packed_memo: bool,
}

impl CounterSolver {
    fn new(joltage: &[u32], instructions: &[Vec<usize>]) -> Self {
        let n = joltage.len();

        let mut masks: Vec<u16> = instructions
            .iter()
            .filter_map(|instr| {
                let mask = instr
                    .iter()
                    .filter(|&&idx| idx < n && idx < 16)
                    .fold(0u16, |acc, &idx| acc | (1u16 << idx));
                (mask != 0).then_some(mask)
            })
            .collect();

        masks.sort_by_key(|&m| std::cmp::Reverse(m.count_ones()));
        masks.dedup();

        let sizes: Vec<u32> = masks.iter().map(|&m| m.count_ones()).collect();
        let initial_rem: Vec<u16> = joltage
            .iter()
            .map(|&v| u16::try_from(v).unwrap_or(u16::MAX))
            .collect();
        let sum_target: u32 = joltage.iter().sum();

        const BITS_PER_REG: u32 = 10;
        let max_target = joltage.iter().copied().max().unwrap_or(0);
        let use_packed_memo = n <= 12 && max_target < (1 << BITS_PER_REG);

        Self {
            masks,
            sizes,
            initial_rem,
            sum_target,
            use_packed_memo,
        }
    }

    fn solve(&self, progress_prefix: Option<&str>) -> u64 {
        if self.initial_rem.is_empty() {
            return 0;
        }

        if self.masks.is_empty() {
            return if self.initial_rem.iter().all(|&v| v == 0) {
                0
            } else {
                u64::MAX
            };
        }

        let interval = std::env::var("AOC_DAY10_PROGRESS_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_millis(250));

        let mut progress = progress_prefix.map(|p| Progress::new(p, self.masks.len(), interval));
        let mut state = SearchState::new(self.sum_target, self.initial_rem.clone());
        let active0 = initial_active_mask(self.masks.len());

        if self.use_packed_memo {
            let mut memo: HashMap<u128, u32> = HashMap::new();
            self.dfs::<PackedMemo>(active0, &mut state, &mut memo, &mut progress);
        } else {
            let mut memo: HashMap<Vec<u16>, u32> = HashMap::new();
            self.dfs::<VecMemo>(active0, &mut state, &mut memo, &mut progress);
        }

        state.result()
    }

    fn dfs<M: MemoStrategy>(
        &self,
        active: u16,
        state: &mut SearchState,
        memo: &mut M::Map,
        progress: &mut Option<Progress<'_>>,
    ) {
        if state.used >= state.best {
            return;
        }

        if state.is_all_zero() {
            state.update_best(progress);
            return;
        }

        if active == 0 {
            return;
        }

        if let Some(p) = progress.as_mut() {
            p.tick(self.masks.len(), active, state, M::len(memo));
        }

        let (coverable, max_sz) = self.compute_coverage(active);
        if !state.can_be_covered(coverable) {
            return;
        }

        if state.lower_bound(max_sz) >= state.best {
            return;
        }

        let mut propagator = ConstraintPropagator::new(active);
        if !propagator.propagate(&self.masks, &mut state.rem) {
            propagator.restore(&self.masks, &mut state.rem);
            return;
        }

        let used2 = state.used + propagator.forced_used();
        if used2 >= state.best {
            propagator.restore(&self.masks, &mut state.rem);
            return;
        }

        let key = M::make_key(propagator.active, &state.rem);
        if let Some(&prev_used) = M::get(memo, &key) {
            if used2 >= prev_used {
                if let Some(p) = progress.as_mut() {
                    p.memo_prunes += 1;
                }
                propagator.restore(&self.masks, &mut state.rem);
                return;
            }
        }
        M::insert(memo, key, used2);

        if state.is_all_zero() {
            let old_used = state.used;
            state.used = used2;
            state.update_best(progress);
            state.used = old_used;
        } else if propagator.active != 0 {
            self.branch::<M>(propagator.active, used2, state, memo, progress);
        }

        propagator.restore(&self.masks, &mut state.rem);
    }

    fn compute_coverage(&self, active: u16) -> (u16, u32) {
        self.masks
            .iter()
            .zip(&self.sizes)
            .enumerate()
            .filter(|(i, _)| (active & (1u16 << i)) != 0)
            .fold((0u16, 1u32), |(cov, max_sz), (_, (&mask, &sz))| {
                (cov | mask, max_sz.max(sz))
            })
    }

    fn branch<M: MemoStrategy>(
        &self,
        active: u16,
        used: u32,
        state: &mut SearchState,
        memo: &mut M::Map,
        progress: &mut Option<Progress<'_>>,
    ) {
        let instr_idx = self.choose_instruction(active, &state.rem);
        let mask = self.masks[instr_idx];
        let max_use = state.max_applicable(mask);
        let next_active = active & !(1u16 << instr_idx);
        let original_used = state.used;

        for x in (0..=max_use).rev() {
            let new_used = used + x as u32;
            if new_used >= state.best {
                continue;
            }

            if x != 0 {
                state.apply_mask(mask, x);
            }

            state.used = new_used;
            self.dfs::<M>(next_active, state, memo, progress);
            state.used = original_used;

            if x != 0 {
                state.unapply_mask(mask, x);
            }
        }
    }

    fn choose_instruction(&self, active: u16, rem: &[u16]) -> usize {
        let mut best_idx = None;
        let mut best_cover = usize::MAX;

        for (reg, &r) in rem.iter().enumerate() {
            if r == 0 {
                continue;
            }
            let bit = 1u16 << reg;
            let (count, last_i) = self
                .masks
                .iter()
                .enumerate()
                .filter(|&(i, m)| (active & (1u16 << i)) != 0 && (m & bit) != 0)
                .fold((0usize, 0usize), |(cnt, _), (i, _)| (cnt + 1, i));

            if count > 0 && count < best_cover {
                best_cover = count;
                best_idx = Some(last_i);
                if count == 1 {
                    break;
                }
            }
        }

        best_idx.unwrap_or_else(|| active.trailing_zeros() as usize)
    }
}

struct SearchState {
    rem: Vec<u16>,
    used: u32,
    best: u32,
    found: bool,
}

impl SearchState {
    fn new(sum_target: u32, rem: Vec<u16>) -> Self {
        Self {
            rem,
            used: 0,
            best: sum_target,
            found: false,
        }
    }

    fn is_all_zero(&self) -> bool {
        self.rem.iter().all(|&v| v == 0)
    }

    fn sum_rem(&self) -> u32 {
        self.rem.iter().map(|&v| v as u32).sum()
    }

    fn max_rem(&self) -> u32 {
        self.rem.iter().copied().max().unwrap_or(0) as u32
    }

    fn can_be_covered(&self, coverable: u16) -> bool {
        self.rem
            .iter()
            .enumerate()
            .all(|(i, &r)| r == 0 || (coverable & (1u16 << i)) != 0)
    }

    fn lower_bound(&self, max_sz: u32) -> u32 {
        let lb1 = self.used.saturating_add(self.max_rem());
        let lb2 = self
            .used
            .saturating_add((self.sum_rem() + max_sz - 1) / max_sz);
        lb1.max(lb2)
    }

    fn max_applicable(&self, mask: u16) -> u16 {
        self.rem
            .iter()
            .enumerate()
            .filter(|(i, _)| (mask & (1u16 << i)) != 0)
            .map(|(_, &r)| r)
            .min()
            .unwrap_or(0)
    }

    fn apply_mask(&mut self, mask: u16, amount: u16) {
        for (i, r) in self.rem.iter_mut().enumerate() {
            if (mask & (1u16 << i)) != 0 {
                *r -= amount;
            }
        }
    }

    fn unapply_mask(&mut self, mask: u16, amount: u16) {
        for (i, r) in self.rem.iter_mut().enumerate() {
            if (mask & (1u16 << i)) != 0 {
                *r += amount;
            }
        }
    }

    fn update_best(&mut self, progress: &mut Option<Progress<'_>>) {
        if self.used < self.best {
            self.best = self.used;
            self.found = true;
            if let Some(p) = progress.as_mut() {
                p.report_best(self.used, self.best);
            }
        }
    }

    fn result(&self) -> u64 {
        if self.found {
            self.best as u64
        } else {
            u64::MAX
        }
    }
}

struct ConstraintPropagator {
    active: u16,
    forced_stack: Vec<(usize, u16)>,
}

impl ConstraintPropagator {
    fn new(active: u16) -> Self {
        Self {
            active,
            forced_stack: Vec::new(),
        }
    }

    fn propagate(&mut self, masks: &[u16], rem: &mut [u16]) -> bool {
        loop {
            let mut changed = false;

            for reg in 0..rem.len() {
                let need = rem[reg];
                if need == 0 {
                    continue;
                }

                let bit = 1u16 << reg;
                let covering: Vec<usize> = masks
                    .iter()
                    .enumerate()
                    .filter(|&(i, m)| (self.active & (1u16 << i)) != 0 && (m & bit) != 0)
                    .map(|(i, _)| i)
                    .collect();

                match covering.len() {
                    0 => return false,
                    1 => {
                        let mask_idx = covering[0];
                        let mask = masks[mask_idx];

                        if rem
                            .iter()
                            .enumerate()
                            .any(|(j, &r)| (mask & (1u16 << j)) != 0 && r < need)
                        {
                            return false;
                        }

                        for (j, r) in rem.iter_mut().enumerate() {
                            if (mask & (1u16 << j)) != 0 {
                                *r -= need;
                            }
                        }
                        self.forced_stack.push((mask_idx, need));
                        self.active &= !(1u16 << mask_idx);
                        changed = true;
                    }
                    _ => {}
                }
            }

            if !changed {
                break;
            }
        }
        true
    }

    fn restore(&self, masks: &[u16], rem: &mut [u16]) {
        for &(mask_idx, amount) in self.forced_stack.iter().rev() {
            let mask = masks[mask_idx];
            for (j, r) in rem.iter_mut().enumerate() {
                if (mask & (1u16 << j)) != 0 {
                    *r += amount;
                }
            }
        }
    }

    fn forced_used(&self) -> u32 {
        self.forced_stack.iter().map(|&(_, x)| x as u32).sum()
    }
}

trait MemoStrategy {
    type Key;
    type Map;

    fn make_key(active: u16, rem: &[u16]) -> Self::Key;
    fn get<'a>(map: &'a Self::Map, key: &Self::Key) -> Option<&'a u32>;
    fn insert(map: &mut Self::Map, key: Self::Key, value: u32);
    fn len(map: &Self::Map) -> usize;
}

struct PackedMemo;

impl MemoStrategy for PackedMemo {
    type Key = u128;
    type Map = HashMap<u128, u32>;

    fn make_key(active: u16, rem: &[u16]) -> u128 {
        const BITS_PER_REG: u32 = 10;
        let mut packed: u128 = 0;
        for (i, &r) in rem.iter().enumerate() {
            packed |= (r as u128) << (BITS_PER_REG * i as u32);
        }
        packed | ((active as u128) << (BITS_PER_REG * rem.len() as u32))
    }

    fn get<'a>(map: &'a Self::Map, key: &u128) -> Option<&'a u32> {
        map.get(key)
    }

    fn insert(map: &mut Self::Map, key: u128, value: u32) {
        map.insert(key, value);
    }

    fn len(map: &Self::Map) -> usize {
        map.len()
    }
}

struct VecMemo;

impl MemoStrategy for VecMemo {
    type Key = Vec<u16>;
    type Map = HashMap<Vec<u16>, u32>;

    fn make_key(active: u16, rem: &[u16]) -> Vec<u16> {
        let mut key = Vec::with_capacity(rem.len() + 1);
        key.push(active);
        key.extend_from_slice(rem);
        key
    }

    fn get<'a>(map: &'a Self::Map, key: &Vec<u16>) -> Option<&'a u32> {
        map.get(key)
    }

    fn insert(map: &mut Self::Map, key: Vec<u16>, value: u32) {
        map.insert(key, value);
    }

    fn len(map: &Self::Map) -> usize {
        map.len()
    }
}

fn initial_active_mask(num_masks: usize) -> u16 {
    if num_masks >= 16 {
        u16::MAX
    } else {
        (1u16 << num_masks) - 1
    }
}

struct Progress<'a> {
    prefix: &'a str,
    total_instrs: usize,
    start: Instant,
    last_report: Instant,
    last_best_report: Instant,
    interval: Duration,
    nodes: u64,
    memo_prunes: u64,
}

impl<'a> Progress<'a> {
    fn new(prefix: &'a str, total_instrs: usize, interval: Duration) -> Self {
        let now = Instant::now();
        Self {
            prefix,
            total_instrs,
            start: now,
            last_report: now,
            last_best_report: now,
            interval,
            nodes: 0,
            memo_prunes: 0,
        }
    }

    fn tick(&mut self, total: usize, active: u16, state: &SearchState, memo_len: usize) {
        self.nodes += 1;
        if (self.nodes & 0x0fff) != 0 {
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_report) < self.interval {
            return;
        }
        self.last_report = now;

        let active_count = active.count_ones() as usize;
        let depth = total - active_count;
        eprintln!(
            "[{}] nodes={} depth={}/{} active={} used={} best={} rem_sum={} rem_max={} memo={} prunes={} elapsed={:?}",
            self.prefix,
            self.nodes,
            depth,
            self.total_instrs,
            active_count,
            state.used,
            state.best,
            state.sum_rem(),
            state.max_rem(),
            memo_len,
            self.memo_prunes,
            self.start.elapsed()
        );
    }

    fn report_best(&mut self, used: u32, best: u32) {
        let now = Instant::now();
        if now.duration_since(self.last_best_report) < Duration::from_millis(250) {
            return;
        }
        self.last_best_report = now;
        eprintln!(
            "[{}] new best={} (used={}) nodes={} elapsed={:?}",
            self.prefix,
            best,
            used,
            self.nodes,
            self.start.elapsed()
        );
    }
}

struct PuzzleLine {
    goal: Vec<bool>,
    instructions: Vec<Vec<usize>>,
    joltage: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<PuzzleLine> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> PuzzleLine {
    let mut tokens = line.split_whitespace();

    let goal_str = tokens.next().expect("Missing goal");
    let goal = goal_str[1..goal_str.len() - 1]
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unexpected char in goal: {}", c),
        })
        .collect();

    let instructions: Vec<Vec<usize>> = tokens
        .clone()
        .take_while(|s| !s.starts_with('{'))
        .map(|s| {
            s.trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .map(|n| n.parse().expect("Invalid instruction index"))
                .collect()
        })
        .collect();

    let joltage_str = line
        .split_whitespace()
        .find(|s| s.starts_with('{'))
        .expect("Missing joltage section");

    let joltage = joltage_str[1..joltage_str.len() - 1]
        .split(',')
        .map(|n| n.parse().expect("Invalid joltage value"))
        .collect();

    PuzzleLine {
        goal,
        instructions,
        joltage,
    }
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

        assert_eq!(parsed[0].goal, vec![false, true, true, false]);
        assert_eq!(
            parsed[0].instructions,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
        assert_eq!(parsed[0].joltage, vec![3, 5, 4, 7]);

        assert_eq!(parsed[1].goal, vec![false, false, false, true, false]);
        assert_eq!(
            parsed[1].instructions,
            vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4]
            ]
        );
        assert_eq!(parsed[1].joltage, vec![7, 5, 12, 7, 2]);
    }

    #[test]
    fn test_min_counter_operations() {
        let input = include_str!("../data/day10_example.txt");
        let parsed = parse_input(input);

        assert_eq!(
            min_counter_operations(&parsed[0].joltage, &parsed[0].instructions),
            10
        );
        assert_eq!(
            min_counter_operations(&parsed[1].joltage, &parsed[1].instructions),
            12
        );
        assert_eq!(
            min_counter_operations(&parsed[2].joltage, &parsed[2].instructions),
            11
        );
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../data/day10_example.txt");
        assert_eq!(part2(input), 33);
    }
}
