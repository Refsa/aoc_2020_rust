use std::fs::File;
use std::io::prelude::*;
use std::{io::BufReader, ops::RangeInclusive};

pub fn aoc_4(reader: BufReader<File>) -> String {
    let mut unparsed_records = Vec::new();
    let mut unparsed_record = "".to_string();
    let mut count = 0;
    for line in reader.lines() {
        let buf = line.unwrap();

        if buf.is_empty() {
            // println!("{}", unparsed_record);
            unparsed_records.push(unparsed_record);
            unparsed_record = "".to_string();
            count += 1;
            continue;
        }

        unparsed_record = unparsed_record + buf.as_str() + " ";
    }

    if !unparsed_record.is_empty() {
        unparsed_records.push(unparsed_record);
        count += 1;
    }

    let mut records = Vec::new();
    for record in unparsed_records {
        if let Some(r) = Record::parse_record(record) {
            // println!("{:?}", r);
            records.push(r);
        }
    }

    let mut valid_passports = 0u64;
    for record in &records {
        if record.validate() {
            valid_passports += 1;
        }
    }

    format!("{}", valid_passports)
}

#[derive(Default, Debug)]
struct Record {
    eyr: u64,
    byr: u64,
    iyr: u64,
    hgt: Option<(u64, String)>,
    hcl: Option<(String, u64)>,
    ecl: String,
    pid: u64,
    cid: u64,
}

impl Record {
    pub fn validate(&self) -> bool {
        // println!("{:?}", self);
        self.eyr != 0
            && self.byr != 0
            && self.iyr != 0
            && self.hgt.is_some()
            && self.pid != 0
            && self.hcl.is_some()
            && !self.ecl.is_empty()
    }
}

trait RecordParser {
    fn parse_record(input: String) -> Option<Record>;
    fn parse_record_content(input: &str, record: &mut Record);
}

impl RecordParser for Record {
    fn parse_record(input: String) -> Option<Record> {
        let mut record = Default::default();

        for (_, seg) in input.split(' ').enumerate() {
            Self::parse_record_content(seg, &mut record);
        }

        Some(record)
    }

    fn parse_record_content(input: &str, record: &mut Record) {
        let mut input = input.split(':');
        let input = (input.nth(0), input.nth(0));

        if let (Some(key), Some(val)) = input {
            match key {
                "byr" => record.byr = parse_range(val, 1920..=2002),
                "eyr" => record.eyr = parse_range(val, 2020..=2030),
                "iyr" => record.iyr = parse_range(val, 2010..=2020),
                "hgt" => record.hgt = parse_hgt(val),
                "cid" => record.cid = 0,
                "pid" => record.pid = parse_pid(val),
                "hcl" => record.hcl = parse_hcl(val),
                "ecl" => record.ecl = parse_ecl(val),
                _ => (),
            }
        }
    }
}

fn parse_ecl(input: &str) -> String {
    match input {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => input.to_string(),
        _ => "".to_string()
    }
}

fn parse_hcl(input: &str) -> Option<(String, u64)> {
    if input.starts_with('#') {
        let mut valid = true;
        for (_, c) in input.chars().enumerate().skip(1) {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' => (),
                _ => valid = false,
            }
        }
        
        if valid {
            return Some(("#".to_string(), 0));
        }
    }

    None
}

fn parse_pid(input: &str) -> u64 {
    let input = input.trim();

    match input.parse::<u64>() {
        Ok(val) => {
            if input.len() == 9 {
                val
            } else {
                0
            }
        }
        Err(_) => 0
    }
}

fn parse_hgt(input: &str) -> Option<(u64, String)> {
    if let Ok(val) = input.trim_end_matches(char::is_alphabetic).parse::<u64>() {
        if input.ends_with("cm") {
            if (150..=193).contains(&val) {
                return Some((val, "cm".to_string()));
            }
        }
    
        if input.ends_with("in") {
            if (59..=76).contains(&val) {
                return Some((val, "in".to_string()));
            }
        }
    }

    None
}

fn parse_u64(input: &str) -> u64 {
    match input.parse::<u64>() {
        Ok(val) => val,
        Err(_) => {
            println!("--- {}", input);
            0
        }
    }
}

fn parse_range(input: &str, range: RangeInclusive<u64>) -> u64 {
    let input = parse_u64(input);
    if input != 0 && range.contains(&input) {
        return input;
    }

    0
}

fn parse_str(input: &str) -> String {
    input.to_owned()
}
