use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

pub fn aoc_8(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let ops = vec!["acc", "jmp", "nop"];

    let mut accumulator = 0i64;
    let mut current_line = 0usize;
    let mut ran_ops = HashSet::new();

    loop {
        let (op, offset) = lines[current_line].split_at(3);
        let mut offset = offset.chars();
        let sign = if offset.nth(0).unwrap() == '+' {-1} else {1};
        let num: i64 = offset.as_str().parse().unwrap();
        let num = num * sign;
        
        if !ran_ops.insert(current_line) {
            break;
        }

        match op {
            "jmp" => current_line = (current_line as i64 + num) as usize,
            "acc" => {
                accumulator += num;
                current_line += 1;
            },
            "nop" => {
                current_line += 1;
            },
            _ => (),
        }

        // println!("{} - {}", op, num);
    }

    format!("{}", accumulator)
}