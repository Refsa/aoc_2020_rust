use std::io::BufReader;
use std::{io::BufRead, fs::File};

pub fn aoc_1(reader: BufReader<File>) -> String {
    let nums: Vec<u32> = reader
        .lines()
        .into_iter()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect();

    let val: u32 = {
        let mut _val = 0;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                for k in j+1..nums.len() {
                    let sum = nums[i] + nums[j] + nums[k];
                    if sum == 2020 {
                        _val = nums[i] * nums[j] * nums[k];
                        break;
                    }
                }
            }
            if _val != 0 {
                break;
            }
        }
        _val
    };

    format!("val: {}", val)
}
