use std::io::BufReader;
use std::{fs::File, io::BufRead};
use std::{ops::RangeInclusive};

pub fn aoc_2(reader: BufReader<File>) -> String {
    let mut first_part_count = 0;
    let mut second_part_count = 0;

    for line in reader.lines().into_iter() {
        let buf = line.unwrap();

        if first_part(&buf) {
            first_part_count += 1;
        }

        if second_part(&buf) {
            second_part_count += 1;
        }
    }

    format!("{}\n\t{}", first_part_count, second_part_count)
}

fn second_part(line: &String) -> bool {
    let parts: Vec<&str> = line.split(':').collect();

    let password = parts[1].trim();

    let needed: Vec<&str> = parts[0].split(' ').collect();
    let needed_char: char = needed[1].trim().chars().next().unwrap();
    let needed_count: Vec<&str> = needed[0].trim().split('-').collect();

    let first_index: usize = needed_count[0].parse::<usize>().unwrap() - 1;
    let second_index: usize = needed_count[1].parse::<usize>().unwrap() - 1;

    let first_char = password.chars().nth(first_index).unwrap() == needed_char;
    let second_char = password.chars().nth(second_index).unwrap() == needed_char;

    let has = first_char as u32 + second_char as u32;

    has == 1
}

fn first_part(line: &String) -> bool {
    let parts: Vec<&str> = line.split(':').collect();

    let password = parts[1].trim();

    let needed: Vec<&str> = parts[0].split(' ').collect();
    let needed_char: char = needed[1].trim().chars().next().unwrap();
    let needed_count: Vec<&str> = needed[0].trim().split('-').collect();

    let needed_range: RangeInclusive<u32> =
        needed_count[0].parse().unwrap()..=needed_count[1].parse().unwrap();

    let found = password
        .chars()
        .into_iter()
        .filter(|c| *c == needed_char)
        .count() as u32;

    needed_range.contains(&found)
}
