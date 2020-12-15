use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn swap_insert(tuple: &mut (usize, usize), next: usize) {
    tuple.1 = tuple.0;
    tuple.0 = next;
}

pub fn aoc_15(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let nums = lines[0]
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect::<Vec<usize>>();

    let sw = std::time::Instant::now();
    for _ in 0..10000 {
        let part1 = solve_part(&nums, 2020);
    }
    let elapsed_part1 = sw.elapsed().as_micros() / 10000;

    let sw = std::time::Instant::now();
    for _ in 0..5 {
        let part2 = solve_part(&nums, 30000000);
    }
    let elapsed_part2 = sw.elapsed().as_millis() / 5;

    let part1 = solve_part(&nums, 2020);
    assert_eq!(410, part1);
    let part2 = solve_part(&nums, 30000000);
    assert_eq!(238, part2);

    format!(
        "Part 1 (~{}µs): {}\n\tPart 2 (~{}ms): {}",
        elapsed_part1, part1, elapsed_part2, part2
    )
}

fn solve_part(nums: &Vec<usize>, stop_at: usize) -> usize {
    let mut spoken = vec![(0usize, 0usize); stop_at];
    for (i, num) in nums.iter().enumerate() {
        spoken[*num] = (i + 1, 0);
    }

    let mut last_number = *nums.last().unwrap();
    for i in nums.len() + 1..=stop_at {
        let current = spoken[last_number];

        let diff = match current.1 {
            0 => 0,
            _ => current.0 - current.1,
        };

        swap_insert(&mut spoken[diff], i);
        last_number = diff;
    }

    last_number
}
