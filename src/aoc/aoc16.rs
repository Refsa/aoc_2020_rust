use std::{collections::HashSet, usize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default, Debug)]
struct Either<T> {
    a: T,
    b: T,
}

impl Either<(u32, u32)> {
    pub fn pick(&self, num: u32) -> Option<(u32, u32)> {
        if num >= self.a.0 && num <= self.a.1 {
            Some(self.a)
        } else if num >= self.b.0 && num <= self.b.1 {
            Some(self.b)
        } else {
            None
        }
    }
    pub fn parse(input: &str) -> Either<(u32, u32)> {
        let (a, b) = input.split_once(" or ").unwrap();

        let (a0, a1) = a.split_once("-").unwrap();
        let (b0, b1) = b.split_once("-").unwrap();

        Either {
            a: (a0.parse().unwrap(), a1.parse().unwrap()),
            b: (b0.parse().unwrap(), b1.parse().unwrap()),
        }
    }
}

fn parse_ticket(input: &str) -> Vec<u32> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

pub fn aoc_16(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let rules: Vec<_> = lines
        .iter()
        .take(20)
        .map(|l| Either::parse(l.split(": ").nth(1).unwrap()))
        .collect();

    let my_ticket = parse_ticket(&lines[22]);
    let mut other_tickets = Vec::new();
    for i in 25..lines.len() {
        other_tickets.push(parse_ticket(&lines[i]));
    }

    let part1 = part_1(&other_tickets, &rules);
    assert_eq!(26009, part1);

    // let part2 = part_2(other_tickets, &field_ranges, &my_ticket);
    let part2 = part_2(other_tickets, my_ticket, rules);
    // assert_eq!(12 * 11 * 13, part2);

    format!("Part 1: {}\n\tPart 2: {}", part1, part2)
}

fn part_1(tickets: &Vec<Vec<u32>>, rules: &Vec<Either<(u32, u32)>>) -> u32 {
    let mut invalid_sum = 0;

    for ticket in tickets {
        let valid = ticket
            .iter()
            .find(|v| !rules.iter().any(|r| r.pick(**v).is_some()));

        if let Some(invalid) = valid {
            invalid_sum += *invalid;
        }
    }

    invalid_sum
}

fn part_2(tickets: Vec<Vec<u32>>, my_ticket: Vec<u32>, rules: Vec<Either<(u32, u32)>>) -> usize {
    let mut tickets: Vec<_> = tickets
        .iter()
        .filter(|t| {
            t.iter()
                .all(|f| rules.iter().any(|r| r.pick(*f).is_some()))
        })
        .collect();
    tickets.push(&my_ticket);

    let mut seen = HashSet::new();
    let indices = rules.iter().map(|rule| {
        for j in 0..my_ticket.len() {
            if seen.contains(&j) {
                continue;
            }
            if tickets.iter().all(|v| rule.pick(v[j]).is_some()) {
                seen.insert(j);
                return j;
            }
        }
        0
    });

    let prod = indices
        .take(6)
        .map(|i| my_ticket[i])
        .fold(1usize, |acc, v| acc * v as usize);

    prod
}
