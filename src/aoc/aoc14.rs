use gcd::*;
use std::{cmp::Ordering, collections::HashMap, f32, i32};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

pub fn aoc_14(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let part1 = part_1(&lines);
    assert_eq!(5875750429995, part1);

    let part1 = part_2(&lines);

    format!("Part1: {}", part1)
}

struct ChipV2 {
    memory: HashMap<u64, u64>,
}

fn part_2(lines: &Vec<String>) -> u64 {
    let mut programs = Vec::new();
    let mut program = Program::default();
    for line in lines {
        if line.starts_with("mask") {
            if program.mems.len() != 0 {
                programs.push(program);
            }

            program = Program::default();
            let (_, mask) = line.split_once('=').unwrap();
            program.mask = mask.trim().to_string();
            continue;
        }

        let (mem, value) = line.split_once('=').unwrap();
        let value: u64 = value.trim().parse().unwrap();
        let mem: u64 = mem
            .split("[")
            .nth(1)
            .unwrap()
            .trim_end_matches("] ")
            .parse()
            .unwrap();
        program.mems.push((mem, value));
    }



    0
}

#[derive(Default, Debug)]
struct Program {
    mask: String,
    mems: Vec<(u64, u64)>,
}

pub fn part_1(lines: &Vec<String>) -> u64 {
    let mut programs = Vec::new();

    let mut program = Program::default();
    for line in lines {
        if line.starts_with("mask") {
            if program.mems.len() != 0 {
                programs.push(program);
            }

            program = Program::default();
            let (_, mask) = line.split_once('=').unwrap();
            program.mask = mask.trim().to_string();
            continue;
        }

        let (mem, value) = line.split_once('=').unwrap();
        let value: u64 = value.trim().parse().unwrap();
        let mem: u64 = mem
            .split("[")
            .nth(1)
            .unwrap()
            .trim_end_matches("] ")
            .parse()
            .unwrap();
        program.mems.push((mem, value));
    }
    programs.push(program);

    for program in programs.iter_mut() {
        for (index, value) in program.mems.iter_mut() {
            for (i, c) in program.mask.chars().rev().enumerate() {
                if c == 'X' {
                    // let bit: u64 = 1 << i;
                    // let value = *value & bit;
                    // flag = value | flag;
                } else {
                    let bit: u64 = c.to_digit(10).unwrap() as u64;
                    if bit != 0 {
                        let bit = 1 << i;
                        *value |= bit;
                    } else {
                        let bit = 1 << i;
                        *value &= !bit;
                    }
                }
            }
        }
    }

    let mut set_mems = HashMap::new();
    for (mem, val) in programs.iter().map(|p| p.mems.clone()).flatten() {
        set_mems.insert(mem, val);
    }

    let mut sum = 0;
    for (k, v) in set_mems {
        sum += v;
    }

    sum
}
