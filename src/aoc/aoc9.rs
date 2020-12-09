use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn aoc_9(reader: BufReader<File>) -> String {
    let lines: Vec<u64> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    let preamble_size = 25;

    let mut invalid_number = 0;

    let sw = std::time::Instant::now();
    for _ in 0..1000 {
        for i in preamble_size..lines.len() {
            let preamble = &lines[i - preamble_size..i];
            let current = lines[i];

            let mut valid = false;
            for j in 0..preamble_size {
                for k in j + 1..preamble_size {
                    if preamble[j] + preamble[k] == current {
                        valid = true;
                        break;
                    }
                    if valid {
                        break;
                    }
                }
            }

            if !valid {
                invalid_number = current;
                break;
            }
        }
    }
    let part1_time = format!("{} µs", sw.elapsed().as_millis());

    assert_eq!(18272118, invalid_number);

    let mut decrypt = 0;

    let sw = std::time::Instant::now();
    for _ in 0..1000 {
        for i in 0..lines.len() {
            let mut sum = lines[i];
            for j in i + 1..lines.len() {
                sum += lines[j];

                if sum > invalid_number {
                    break;
                } else if sum == invalid_number {
                    let mut smallest = 99999999999;
                    let mut largest = 0;
                    for k in i..=j {
                        if lines[k] > largest {
                            largest = lines[k];
                        }
                        if lines[k] < smallest {
                            smallest = lines[k];
                        }
                    }
                    decrypt = smallest + largest;
                    break;
                }
            }

            if decrypt != 0 {
                break;
            }
        }
    }
    let part2_time = format!("{} µs", sw.elapsed().as_millis());

    assert_eq!(2186361, decrypt);

    format!("P1 (~{}): {}\n\tP2 (~{}): {}", part1_time, invalid_number, part2_time, decrypt)
}
