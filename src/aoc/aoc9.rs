use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const WINDOW_SIZE: usize = 25;

pub fn aoc_9(reader: BufReader<File>) -> String {
    let lines: Vec<u64> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    let sw = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = solve_part1(&lines);
    }
    let part1_time = format!("{} µs", sw.elapsed().as_micros() / 1000);

    let invalid_number = solve_part1(&lines);
    assert_eq!(18272118, invalid_number);

    let sw = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = solve_part2(&lines, invalid_number);
    }
    let part2_time = format!("{} ns", sw.elapsed().as_nanos() / 1000);

    let decrypt = solve_part2(&lines, invalid_number).unwrap_or_default();
    assert_eq!(2186361, decrypt);

    format!(
        "P1 (~{}): {}\n\tP2 (~{}): {}",
        part1_time, invalid_number, part2_time, decrypt
    )
}

fn solve_part1(lines: &Vec<u64>) -> u64 {
    let invalid_number = lines
        .windows(WINDOW_SIZE + 1)
        .try_find(|&w| Some(validate_line(w, w[WINDOW_SIZE]).is_none()))
        .unwrap_or(None)
        .unwrap_or(&[0u64]);
    let invalid_number = invalid_number[WINDOW_SIZE];
    invalid_number
}

fn validate_line(window: &[u64], current: u64) -> Option<u64> {
    for j in 0..WINDOW_SIZE {
        for k in j + 1..WINDOW_SIZE {
            if window[j] + window[k] == current {
                return Some(current);
            }
        }
    }
    None
}

fn solve_part2(lines: &Vec<u64>, invalid_number: u64) -> Option<u64> {
    let mut sum = 0;
    for i in 0..lines.len() {
        sum = lines[i];
        for j in i + 1..lines.len() {
            sum += lines[j];

            if sum > invalid_number {
                break;
            } else if sum == invalid_number {
                return Some(find_sum(lines, i, j));
            }
        }
    }

    None
}

fn find_sum(lines: &Vec<u64>, start: usize, end: usize) -> u64 {
    let (min, max) = lines
        .iter()
        .skip(start)
        .take(end - start)
        .fold((1 << 63, 0), |a, &v| (a.0.min(v), a.1.max(v)));

    min + max
}
