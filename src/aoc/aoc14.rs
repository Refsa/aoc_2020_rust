use gcd::*;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    f32, i32,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

pub fn aoc_14(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut chipv1: Box<dyn Chip<MemoryStorage = HashMap<u64, u64>, Ops = Ops>> = ChipV1::init();
    chipv1.run(&lines);
    let part1 = chipv1.get_memory().count();
    assert_eq!(5875750429995, part1);

    let mut chipv2: Box<dyn Chip<MemoryStorage = HashMap<u64, u64>, Ops = Ops>> = ChipV2::init();
    chipv2.run(&lines);
    let part2 = chipv2.get_memory().count();
    assert_eq!(5272149590143, part2);

    format!("Part1: {}\n\tPart2: {}", part1, part2)
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

trait Memory {
    fn set(&mut self, loc: u64, val: u64);
    fn count(&self) -> u64;
    fn init() -> Box<Self>
    where
        Self: Sized;
}

impl Memory for HashMap<u64, u64> {
    fn set(&mut self, loc: u64, val: u64) {
        self.insert(loc, val);
    }

    fn count(&self) -> u64 {
        self.values().sum()
    }

    fn init() -> Box<HashMap<u64, u64>> {
        Box::new(HashMap::new())
    }
}

trait Chip {
    type MemoryStorage;
    type Ops;

    fn init() -> Box<Self>
    where
        Self: Sized;
    fn get_memory(&self) -> &dyn Memory;
    fn handle_memory_op(&mut self, loc: u64, val: u64, mask: &String);
}

impl dyn Chip<MemoryStorage = HashMap<u64, u64>, Ops = Ops> {
    pub fn run(&mut self, lines: &Vec<String>) {
        let mut last_mask_op = "".to_string();
        for line in lines {
            match parse_line(line) {
                Ops::Mask(mask) => {
                    last_mask_op = mask;
                }
                Ops::MemoryOp((loc, val)) => {
                    self.handle_memory_op(loc, val, &last_mask_op);
                }
            }
        }
    }
}

struct ChipV2 {
    memory: Box<dyn Memory>,
}

impl Chip for ChipV2 {
    type MemoryStorage = HashMap<u64, u64>;
    type Ops = Ops;

    fn handle_memory_op(&mut self, loc: u64, mut val: u64, mask: &String) {
        let mut flag = loc;
        for (i, c) in mask.chars().rev().enumerate() {
            if c != 'X' {
                let bit: u64 = c.to_digit(10).unwrap() as u64;
                if bit == 1 {
                    let bit = 1 << i;
                    flag |= bit;
                }
            }
        }

        let mut floating = HashSet::new();
        floating.insert(flag);
        generate_memory_addresses(mask, 0, flag, &mut floating);

        for addr in floating {
            self.memory.set(addr, val);
        }

    }

    fn get_memory(&self) -> &dyn Memory {
        self.memory.as_ref()
    }

    fn init() -> Box<Self> {
        Box::new(ChipV2 {
            memory: Self::MemoryStorage::init(),
        })
    }
}

fn generate_memory_addresses(mask: &str, index: usize, flag: u64, set: &mut HashSet<u64>) {
    for (i, c) in mask[index..].chars().rev().enumerate() {
        if c == 'X' {
            let variation = flag ^ (1 << i);

            if set.insert(variation) {
                generate_memory_addresses(mask, index + 1, variation, set);
            }
            if set.insert(flag) {
                generate_memory_addresses(mask, index + 1, flag, set);
            }
        }
    }
}

struct ChipV1 {
    memory: Box<dyn Memory>,
}

impl Chip for ChipV1 {
    type MemoryStorage = HashMap<u64, u64>;
    type Ops = Ops;

    fn handle_memory_op(&mut self, loc: u64, mut val: u64, mask: &String) {
        for (i, c) in mask.chars().rev().enumerate() {
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
        self.memory.set(loc, val);
    }

    fn get_memory(&self) -> &dyn Memory {
        self.memory.as_ref()
    }

    fn init() -> Box<Self> {
        Box::new(ChipV1 {
            memory: Self::MemoryStorage::init(),
        })
    }
}

fn print_mask(mask: u64) {
    for i in (0..36u64).rev() {
        let set = mask & (1 << i);
        if set == 0 {
            print!("{}", 0);
        } else {
            print!("{}", 1);
        }
    }
    println!(" {}", mask);
}
