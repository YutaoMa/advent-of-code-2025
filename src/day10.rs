use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

pub fn run() {
    let input = include_str!("../data/day10_real.txt");
    println!("Day 10 Part 1: {}", part1(input));
    println!("Day 10 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let lines = parse_input(input);
    lines
        .into_iter()
        .map(|(goal, instructions, _)| min_operations(&goal, &instructions))
        .sum()
}

fn part2(input: &str) -> u64 {
    let lines = parse_input(input);
    let total = lines.len();
    let progress_enabled = std::env::var("AOC_DAY10_PROGRESS")
        .ok()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));
    let limit_lines: Option<usize> = std::env::var("AOC_DAY10_LIMIT_LINES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0);

    let total = limit_lines.unwrap_or(total).min(total);
    let threads: usize = std::env::var("AOC_DAY10_THREADS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&t| t > 0)
        .unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        });

    let threaded = threads > 1;
    let per_line_progress = progress_enabled && !threaded;
    if progress_enabled && threaded {
        eprintln!(
            "AOC_DAY10_PROGRESS is enabled, but AOC_DAY10_THREADS>1: disabling per-line progress logs (they interleave). Set AOC_DAY10_THREADS=1 to debug a slow line."
        );
    }

    let compute_one = |i: usize| -> u64 {
        let (_, instructions, joltage) = &lines[i];
        let line_start = Instant::now();
        eprintln!(
            "Processing line {}/{} (joltage: {:?})",
            i + 1,
            total,
            joltage
        );
        let result = if per_line_progress {
            let prefix = format!("line {}/{}", i + 1, total);
            min_counter_operations_with_progress(&prefix, joltage, instructions)
        } else {
            min_counter_operations(joltage, instructions)
        };
        eprintln!(
            "  -> result: {} (elapsed: {:?})",
            result,
            line_start.elapsed()
        );
        if result == u64::MAX {
            panic!("No solution found for line {}/{}", i + 1, total);
        }
        result
    };

    let sum_u128: u128 = if !threaded {
        (0..total).map(|i| compute_one(i) as u128).sum()
    } else {
        eprintln!(
            "Running Day 10 Part 2 with {} threads ({} lines)",
            threads, total
        );
        let next = AtomicUsize::new(0);
        std::thread::scope(|s| {
            let mut handles = Vec::with_capacity(threads);
            for _ in 0..threads {
                handles.push(s.spawn(|| {
                    let mut local_sum: u128 = 0;
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= total {
                            break;
                        }
                        local_sum += compute_one(i) as u128;
                    }
                    local_sum
                }));
            }
            handles
                .into_iter()
                .map(|h| h.join().expect("worker thread panicked"))
                .sum::<u128>()
        })
    };

    u64::try_from(sum_u128)
        .unwrap_or_else(|_| panic!("part2 sum overflowed u64 (sum={})", sum_u128))
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

fn min_counter_operations_with_progress(
    prefix: &str,
    joltage: &[u32],
    instructions: &[Vec<usize>],
) -> u64 {
    min_counter_operations_impl(Some(prefix), joltage, instructions)
}

fn min_counter_operations(joltage: &[u32], instructions: &[Vec<usize>]) -> u64 {
    min_counter_operations_impl(None, joltage, instructions)
}

fn min_counter_operations_impl(
    prefix: Option<&str>,
    joltage: &[u32],
    instructions: &[Vec<usize>],
) -> u64 {
    let n = joltage.len();
    if n == 0 {
        return 0;
    }

    let mut masks: Vec<u16> = instructions
        .iter()
        .filter_map(|instr| {
            let mut mask: u16 = 0;
            for &idx in instr {
                if idx < n && idx < 16 {
                    mask |= 1u16 << idx;
                }
            }
            (mask != 0).then_some(mask)
        })
        .collect();

    if masks.is_empty() {
        return if joltage.iter().all(|&v| v == 0) {
            0
        } else {
            u64::MAX
        };
    }

    masks.sort_by_key(|&m| std::cmp::Reverse(m.count_ones()));

    masks.dedup();

    let m = masks.len();
    let sizes: Vec<u32> = masks.iter().map(|&mm| mm.count_ones()).collect();

    let mut rem: Vec<u16> = joltage
        .iter()
        .map(|&v| u16::try_from(v).unwrap_or(u16::MAX))
        .collect();

    let sum_target: u32 = joltage.iter().map(|&v| v as u32).sum();
    let mut best: u32 = sum_target;
    let mut found = false;

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

        fn tick(
            &mut self,
            depth: usize,
            active: usize,
            used: u32,
            best: u32,
            memo_len: usize,
            rem: &[u16],
        ) {
            self.nodes += 1;
            if (self.nodes & 0x0fff) != 0 {
                return;
            }
            let now = Instant::now();
            if now.duration_since(self.last_report) < self.interval {
                return;
            }
            self.last_report = now;
            let maxr = rem.iter().copied().max().unwrap_or(0);
            let sumr: u32 = rem.iter().map(|&v| v as u32).sum();
            eprintln!(
                "[{}] nodes={} depth={}/{} active={} used={} best={} rem_sum={} rem_max={} memo={} memo_prunes={} elapsed={:?}",
                self.prefix,
                self.nodes,
                depth,
                self.total_instrs,
                active,
                used,
                best,
                sumr,
                maxr,
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

    fn max_rem(rem: &[u16]) -> u32 {
        rem.iter().copied().max().unwrap_or(0) as u32
    }

    fn sum_rem(rem: &[u16]) -> u32 {
        rem.iter().map(|&v| v as u32).sum()
    }

    fn is_all_zero(rem: &[u16]) -> bool {
        rem.iter().all(|&v| v == 0)
    }

    fn popcount16(x: u16) -> usize {
        x.count_ones() as usize
    }

    fn dfs_active_packed(
        masks: &[u16],
        sizes: &[u32],
        active: u16,
        rem: &mut [u16],
        used: u32,
        best: &mut u32,
        found: &mut bool,
        memo: &mut HashMap<u128, u32>,
        pack_state: &impl Fn(u16, &[u16]) -> u128,
        progress: &mut Option<Progress<'_>>,
    ) {
        if used >= *best {
            return;
        }

        if is_all_zero(rem) {
            let old = *best;
            *best = (*best).min(used);
            *found = true;
            if *best < old {
                if let Some(p) = progress.as_mut() {
                    p.report_best(used, *best);
                }
            }
            return;
        }

        if active == 0 {
            return;
        }

        if let Some(p) = progress.as_mut() {
            let active_count = popcount16(active);
            let depth = p.total_instrs - active_count;
            p.tick(depth, active_count, used, *best, memo.len(), rem);
        }

        let mut coverable: u16 = 0;
        let mut max_sz: u32 = 1;
        for i in 0..masks.len() {
            if (active & (1u16 << i)) != 0 {
                coverable |= masks[i];
                max_sz = max_sz.max(sizes[i]);
            }
        }

        for (ri, &r) in rem.iter().enumerate() {
            if r > 0 && (coverable & (1u16 << ri)) == 0 {
                return;
            }
        }

        let lb1 = used.saturating_add(max_rem(rem));
        let srem = sum_rem(rem);
        let lb2 = used.saturating_add((srem + max_sz - 1) / max_sz);
        let lb = lb1.max(lb2);
        if lb >= *best {
            return;
        }

        let mut active_local = active;
        let mut forced_stack: Vec<(usize, u16)> = Vec::new();
        let mut impossible = false;
        'prop: loop {
            let mut changed = false;
            for reg in 0..rem.len() {
                let need = rem[reg];
                if need == 0 {
                    continue;
                }
                let bit = 1u16 << reg;
                let mut count = 0usize;
                let mut last_i = 0usize;
                for i in 0..masks.len() {
                    if (active_local & (1u16 << i)) != 0 && (masks[i] & bit) != 0 {
                        count += 1;
                        last_i = i;
                        if count > 1 {
                            break;
                        }
                    }
                }
                if count == 0 {
                    impossible = true;
                    break 'prop;
                }
                if count == 1 {
                    let mask = masks[last_i];
                    for rj in 0..rem.len() {
                        if (mask & (1u16 << rj)) != 0 && rem[rj] < need {
                            impossible = true;
                            break 'prop;
                        }
                    }
                    for rj in 0..rem.len() {
                        if (mask & (1u16 << rj)) != 0 {
                            rem[rj] -= need;
                        }
                    }
                    forced_stack.push((last_i, need));
                    active_local &= !(1u16 << last_i);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        if impossible {
            for &(i, x) in forced_stack.iter().rev() {
                let mask = masks[i];
                for rj in 0..rem.len() {
                    if (mask & (1u16 << rj)) != 0 {
                        rem[rj] += x;
                    }
                }
            }
            return;
        }

        let forced_used: u32 = forced_stack.iter().map(|&(_, x)| x as u32).sum();
        let used2 = used + forced_used;
        if used2 >= *best {
            for &(i, x) in forced_stack.iter().rev() {
                let mask = masks[i];
                for rj in 0..rem.len() {
                    if (mask & (1u16 << rj)) != 0 {
                        rem[rj] += x;
                    }
                }
            }
            return;
        }

        let key = pack_state(active_local, rem);
        if let Some(&prev_used) = memo.get(&key) {
            if used2 >= prev_used {
                if let Some(p) = progress.as_mut() {
                    p.memo_prunes += 1;
                }
                for &(i, x) in forced_stack.iter().rev() {
                    let mask = masks[i];
                    for rj in 0..rem.len() {
                        if (mask & (1u16 << rj)) != 0 {
                            rem[rj] += x;
                        }
                    }
                }
                return;
            }
        }
        memo.insert(key, used2);

        if is_all_zero(rem) {
            let old = *best;
            *best = (*best).min(used2);
            *found = true;
            if *best < old {
                if let Some(p) = progress.as_mut() {
                    p.report_best(used2, *best);
                }
            }
        } else if active_local != 0 {
            let mut chosen_i: Option<usize> = None;
            let mut chosen_reg_cover = usize::MAX;
            for reg in 0..rem.len() {
                if rem[reg] == 0 {
                    continue;
                }
                let bit = 1u16 << reg;
                let mut cover_count = 0usize;
                let mut some_i = None;
                for i in 0..masks.len() {
                    if (active_local & (1u16 << i)) != 0 && (masks[i] & bit) != 0 {
                        cover_count += 1;
                        some_i = Some(i);
                    }
                }
                if cover_count > 0 && cover_count < chosen_reg_cover {
                    chosen_reg_cover = cover_count;
                    chosen_i = some_i;
                    if cover_count == 1 {
                        break;
                    }
                }
            }
            let i = chosen_i.unwrap_or_else(|| active_local.trailing_zeros() as usize);
            let mask = masks[i];

            let mut max_use: u16 = u16::MAX;
            for rj in 0..rem.len() {
                if (mask & (1u16 << rj)) != 0 {
                    max_use = max_use.min(rem[rj]);
                }
            }

            let next_active = active_local & !(1u16 << i);
            for x in (0..=max_use).rev() {
                let new_used = used2 + x as u32;
                if new_used >= *best {
                    continue;
                }
                if x != 0 {
                    for rj in 0..rem.len() {
                        if (mask & (1u16 << rj)) != 0 {
                            rem[rj] -= x;
                        }
                    }
                }

                dfs_active_packed(
                    masks,
                    sizes,
                    next_active,
                    rem,
                    new_used,
                    best,
                    found,
                    memo,
                    pack_state,
                    progress,
                );

                if x != 0 {
                    for rj in 0..rem.len() {
                        if (mask & (1u16 << rj)) != 0 {
                            rem[rj] += x;
                        }
                    }
                }
            }
        }

        for &(i, x) in forced_stack.iter().rev() {
            let mask = masks[i];
            for rj in 0..rem.len() {
                if (mask & (1u16 << rj)) != 0 {
                    rem[rj] += x;
                }
            }
        }
    }

    const BITS_PER_REG: u32 = 10;
    let max_target = joltage.iter().copied().max().unwrap_or(0);
    let can_pack = n <= 12 && max_target < (1 << BITS_PER_REG);

    let interval = std::env::var("AOC_DAY10_PROGRESS_INTERVAL_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or(Duration::from_millis(250));
    let mut progress = prefix.map(|p| Progress::new(p, m, interval));

    if can_pack {
        let pack_state = |active: u16, remv: &[u16]| -> u128 {
            let mut packed: u128 = 0;
            for (i, &r) in remv.iter().enumerate() {
                packed |= (r as u128) << (BITS_PER_REG * i as u32);
            }
            packed | ((active as u128) << (BITS_PER_REG * remv.len() as u32))
        };

        let mut memo: HashMap<u128, u32> = HashMap::new();
        let active0: u16 = if m == 16 { u16::MAX } else { (1u16 << m) - 1 };
        dfs_active_packed(
            &masks,
            &sizes,
            active0,
            &mut rem,
            0,
            &mut best,
            &mut found,
            &mut memo,
            &pack_state,
            &mut progress,
        );
    } else {
        fn dfs_vecmemo(
            masks: &[u16],
            sizes: &[u32],
            active: u16,
            rem: &mut [u16],
            used: u32,
            best: &mut u32,
            found: &mut bool,
            memo: &mut HashMap<Vec<u16>, u32>,
            progress: &mut Option<Progress<'_>>,
        ) {
            if used >= *best {
                return;
            }

            if is_all_zero(rem) {
                let old = *best;
                *best = (*best).min(used);
                *found = true;
                if *best < old {
                    if let Some(p) = progress.as_mut() {
                        p.report_best(used, *best);
                    }
                }
                return;
            }

            if active == 0 {
                return;
            }

            if let Some(p) = progress.as_mut() {
                let active_count = active.count_ones() as usize;
                let depth = p.total_instrs - active_count;
                p.tick(depth, active_count, used, *best, memo.len(), rem);
            }

            let mut coverable: u16 = 0;
            let mut max_sz: u32 = 1;
            for i in 0..masks.len() {
                if (active & (1u16 << i)) != 0 {
                    coverable |= masks[i];
                    max_sz = max_sz.max(sizes[i]);
                }
            }
            for (ri, &r) in rem.iter().enumerate() {
                if r > 0 && (coverable & (1u16 << ri)) == 0 {
                    return;
                }
            }

            let lb1 = used.saturating_add(max_rem(rem));
            let srem = sum_rem(rem);
            let lb2 = used.saturating_add((srem + max_sz - 1) / max_sz);
            let lb = lb1.max(lb2);
            if lb >= *best {
                return;
            }

            let mut active_local = active;
            let mut forced_stack: Vec<(usize, u16)> = Vec::new();
            let mut impossible = false;
            'prop: loop {
                let mut changed = false;
                for reg in 0..rem.len() {
                    let need = rem[reg];
                    if need == 0 {
                        continue;
                    }
                    let bit = 1u16 << reg;
                    let mut count = 0usize;
                    let mut last_i = 0usize;
                    for i in 0..masks.len() {
                        if (active_local & (1u16 << i)) != 0 && (masks[i] & bit) != 0 {
                            count += 1;
                            last_i = i;
                            if count > 1 {
                                break;
                            }
                        }
                    }
                    if count == 0 {
                        impossible = true;
                        break 'prop;
                    }
                    if count == 1 {
                        let mask = masks[last_i];
                        for rj in 0..rem.len() {
                            if (mask & (1u16 << rj)) != 0 && rem[rj] < need {
                                impossible = true;
                                break 'prop;
                            }
                        }
                        for rj in 0..rem.len() {
                            if (mask & (1u16 << rj)) != 0 {
                                rem[rj] -= need;
                            }
                        }
                        forced_stack.push((last_i, need));
                        active_local &= !(1u16 << last_i);
                        changed = true;
                    }
                }
                if !changed {
                    break;
                }
            }

            if impossible {
                for &(i, x) in forced_stack.iter().rev() {
                    let mask = masks[i];
                    for rj in 0..rem.len() {
                        if (mask & (1u16 << rj)) != 0 {
                            rem[rj] += x;
                        }
                    }
                }
                return;
            }
            let forced_used: u32 = forced_stack.iter().map(|&(_, x)| x as u32).sum();
            let used2 = used + forced_used;
            if used2 >= *best {
                for &(i, x) in forced_stack.iter().rev() {
                    let mask = masks[i];
                    for rj in 0..rem.len() {
                        if (mask & (1u16 << rj)) != 0 {
                            rem[rj] += x;
                        }
                    }
                }
                return;
            }

            let mut key = Vec::with_capacity(rem.len() + 1);
            key.push(active_local);
            key.extend_from_slice(rem);
            if let Some(&prev_used) = memo.get(&key) {
                if used2 >= prev_used {
                    if let Some(p) = progress.as_mut() {
                        p.memo_prunes += 1;
                    }
                    for &(i, x) in forced_stack.iter().rev() {
                        let mask = masks[i];
                        for rj in 0..rem.len() {
                            if (mask & (1u16 << rj)) != 0 {
                                rem[rj] += x;
                            }
                        }
                    }
                    return;
                }
            }
            memo.insert(key, used2);

            if is_all_zero(rem) {
                let old = *best;
                *best = (*best).min(used2);
                *found = true;
                if *best < old {
                    if let Some(p) = progress.as_mut() {
                        p.report_best(used2, *best);
                    }
                }
            } else if active_local != 0 {
                let mut chosen_i: Option<usize> = None;
                let mut chosen_reg_cover = usize::MAX;
                for reg in 0..rem.len() {
                    if rem[reg] == 0 {
                        continue;
                    }
                    let bit = 1u16 << reg;
                    let mut cover_count = 0usize;
                    let mut some_i = None;
                    for i in 0..masks.len() {
                        if (active_local & (1u16 << i)) != 0 && (masks[i] & bit) != 0 {
                            cover_count += 1;
                            some_i = Some(i);
                        }
                    }
                    if cover_count > 0 && cover_count < chosen_reg_cover {
                        chosen_reg_cover = cover_count;
                        chosen_i = some_i;
                        if cover_count == 1 {
                            break;
                        }
                    }
                }
                let i = chosen_i.unwrap_or_else(|| active_local.trailing_zeros() as usize);
                let mask = masks[i];

                let mut max_use: u16 = u16::MAX;
                for rj in 0..rem.len() {
                    if (mask & (1u16 << rj)) != 0 {
                        max_use = max_use.min(rem[rj]);
                    }
                }

                let next_active = active_local & !(1u16 << i);
                for x in (0..=max_use).rev() {
                    let new_used = used2 + x as u32;
                    if new_used >= *best {
                        continue;
                    }
                    if x != 0 {
                        for rj in 0..rem.len() {
                            if (mask & (1u16 << rj)) != 0 {
                                rem[rj] -= x;
                            }
                        }
                    }

                    dfs_vecmemo(
                        masks,
                        sizes,
                        next_active,
                        rem,
                        new_used,
                        best,
                        found,
                        memo,
                        progress,
                    );

                    if x != 0 {
                        for rj in 0..rem.len() {
                            if (mask & (1u16 << rj)) != 0 {
                                rem[rj] += x;
                            }
                        }
                    }
                }
            }

            for &(i, x) in forced_stack.iter().rev() {
                let mask = masks[i];
                for rj in 0..rem.len() {
                    if (mask & (1u16 << rj)) != 0 {
                        rem[rj] += x;
                    }
                }
            }
        }

        let mut memo: HashMap<Vec<u16>, u32> = HashMap::new();
        let active0: u16 = if m == 16 { u16::MAX } else { (1u16 << m) - 1 };
        dfs_vecmemo(
            &masks,
            &sizes,
            active0,
            &mut rem,
            0,
            &mut best,
            &mut found,
            &mut memo,
            &mut progress,
        );
    }

    if found { best as u64 } else { u64::MAX }
}

fn parse_input(input: &str) -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let mut sections = line.split_whitespace();
            let goal = sections.next().unwrap();
            let instruction_sections: Vec<&str> = sections
                .by_ref()
                .take_while(|s| !s.starts_with('{'))
                .collect();
            let joltage = line
                .split_whitespace()
                .find(|s| s.starts_with('{'))
                .expect("Could not find joltage section");
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
                joltage[1..joltage.len() - 1]
                    .split(',')
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
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
        assert_eq!(parsed[0].2, vec![3, 5, 4, 7]);
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
        assert_eq!(parsed[1].2, vec![7, 5, 12, 7, 2]);
    }

    #[test]
    fn test_min_counter_operations() {
        let input = include_str!("../data/day10_example.txt");
        let parsed = parse_input(input);
        assert_eq!(min_counter_operations(&parsed[0].2, &parsed[0].1), 10);
        assert_eq!(min_counter_operations(&parsed[1].2, &parsed[1].1), 12);
        assert_eq!(min_counter_operations(&parsed[2].2, &parsed[2].1), 11);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../data/day10_example.txt");
        assert_eq!(part2(input), 33);
    }
}
