use gcd::*;
use std::{cmp::Ordering, collections::HashMap, f32, i32};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

pub fn aoc_14(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let part1 = ChipV1::run(&lines).count_memory();
    assert_eq!(5875750429995, part1);

    format!("Part1: {}", part1)
}

enum Ops {
    Mask(String),
    MemoryOp((u64, u64)),
}

fn parse_line(line: &String) -> Ops {
    if line.starts_with("mask") {
        let (_, mask) = line.split_once('=').unwrap();
        Ops::Mask(mask.trim().to_string())
    } else {
        let (mem, value) = line.split_once('=').unwrap();
        let value: u64 = value.trim().parse().unwrap();
        let mem: u64 = mem
            .split("[")
            .nth(1)
            .unwrap()
            .trim_end_matches("] ")
            .parse()
            .unwrap();
        Ops::MemoryOp((mem, value))
    }
}

type Memory = HashMap<u64, u64>;

#[derive(Default)]
struct ChipV1 {
    memory: Memory,
}

impl ChipV1 {
    pub fn run(lines: &Vec<String>) -> ChipV1 {
        let mut chip = ChipV1::default();

        let mut last_mask_op = "".to_string();
        for line in lines {
            match parse_line(line) {
                Ops::Mask(mask) => {
                    last_mask_op = mask;
                }
                Ops::MemoryOp((loc, mut val)) => {
                    for (i, c) in last_mask_op.chars().rev().enumerate() {
                        if c != 'X' {
                            let bit: u64 = c.to_digit(10).unwrap() as u64;
                            if bit != 0 {
                                let bit = 1 << i;
                                val |= bit;
                            } else {
                                let bit = 1 << i;
                                val &= !bit;
                            }
                        }
                    }
                    chip.memory.insert(loc, val);
                }
            }
        }

        chip
    }

    pub fn count_memory(&self) -> u64 {
        self.memory.values().sum()
    }
}