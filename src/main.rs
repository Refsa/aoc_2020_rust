#![feature(bool_to_option)]

use std::fs::File;
use std::{io::BufReader, path::Path};

mod aoc;
use aoc::*;

fn main() {
    // run(1, aoc_1);
    // run(2, aoc_2);
    // run(3, aoc_3);
    // run(4, aoc_4);
    // run(5, aoc_5);
    // run(6, aoc_6);
    // run(7, aoc_7);
    run(8, aoc_8);
}

fn load_file(path: &str) -> BufReader<File> {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader
}

fn run(day: u32, runner: DayRunner) {
    let result = runner.run(format!("./inputs/aoc_{}_input.txt", day).as_str());
    println!("AOC {}\n\t{}", day, result);
}

type DayRunner = fn(BufReader<File>) -> String;

pub trait Runner {
    fn run(self, file: &str) -> String;
}

impl Runner for DayRunner {
    fn run(self, file: &str) -> String {
        (self)(load_file(file))
    }
}