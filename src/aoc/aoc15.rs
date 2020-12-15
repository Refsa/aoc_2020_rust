use std::collections::{HashMap, HashSet};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn swap_insert(mut tuple: (usize, usize), next: usize) -> (usize, usize) {
    tuple.1 = tuple.0;
    tuple.0 = next;
    tuple
}

pub fn aoc_15(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let nums = lines[0]
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect::<Vec<usize>>();

    let part1 = part_1(&nums);
    assert_eq!(410, part1);

    let part2 = part_2(&nums);

    format!("Part 1: {}\n\tPart 2: {}", part1, part2)
}

fn part_2(nums: &Vec<usize>) -> usize {
    let mut spoken: HashMap<usize, (usize, usize)> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        spoken.insert(*num, (i + 1, 0));
    }

    let mut last_number = *nums.last().unwrap();
    for i in nums.len() + 1..=30000000 {
        let current = spoken[&last_number];

        let diff = match current.1 {
            0 => 0,
            _ => current.0 - current.1,
        };

        spoken.insert(diff, swap_insert(*spoken.get(&diff).unwrap_or(&(0, 0)), i));

        last_number = diff;
    }

    last_number
}

fn part_1(nums: &Vec<usize>) -> usize {
    let mut spoken = vec![(0usize, 0usize); 65535];
    for (i, num) in nums.iter().enumerate() {
        spoken[*num] = (i + 1, 0);
    }

    let mut last_number = *nums.last().unwrap();
    for i in nums.len() + 1..2021 {
        let current = spoken[last_number];

        let diff = match current.1 {
            0 => 0,
            _ => current.0 - current.1,
        };

        spoken[diff] = swap_insert(spoken[diff], i);
        // println!("{}: {} - {}", i, last_number, diff);

        last_number = diff;
    }

    last_number
}
