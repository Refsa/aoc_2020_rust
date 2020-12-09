use crate::File;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::{collections::HashSet, sync::Arc, thread::JoinHandle};

const SHINY_GOLD: &str = "shiny gold";

const PARSER_BENCHES: u128 = 1000;
const PART1_BENCHES: u128 = 1000;
const PART2_BENCHES: u128 = 10000;

#[derive(Debug, Default, Clone)]
struct Bag {
    contents: HashMap<String, u32>,
}

#[derive(Default, Debug, Clone)]
struct Bags {
    contents: HashMap<String, Bag>,
}

fn digest_content(line: &str) -> (String, u32) {
    let split = line.rmatch_indices(" bag").nth(0).unwrap().0;
    let inner = line.split_at(split).0;

    let (count, color) = inner.split_at(1);
    let color = color.trim_start().to_owned();
    let count: u32 = count.parse().unwrap();

    (color, count)
}

fn parse_contents(reader: &Vec<String>) -> Bags {
    let mut bags: Bags = Default::default();
    reader.iter().for_each(|l| {
        let content: Vec<&str> = l.split(" bags contain ").collect();

        let mut bag = Bag::default();
        if !content[1].contains("no other bags") {
            for (color, count) in content[1].split(", ").map(|i| digest_content(i)) {
                bag.contents.insert(color, count);
            }
        }
        bags.contents.insert(content[0].trim().to_owned(), bag);
    });
    bags
}

pub fn aoc_7(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let sw = std::time::Instant::now();
    for _ in 0..PARSER_BENCHES {
        parse_contents(&lines);
    }
    let parser_time = sw.elapsed().as_micros() / PARSER_BENCHES;

    let bags = parse_contents(&lines);

    let sw = std::time::Instant::now();
    let mut part1 = 0u32;
    for _ in 0..PART1_BENCHES {
        // part1 = part_1(&bags);
        part1 = part_1(&bags);
    }
    let part1_time = sw.elapsed().as_micros() / PART1_BENCHES;
    assert_eq!(part1, 142);

    let sw = std::time::Instant::now();
    let mut part2 = 0u32;
    for _ in 0..PART2_BENCHES {
        let tree = BagTree::construct(&bags, SHINY_GOLD.to_string());
        part2 = tree.part_2();
    }
    let part2_time = sw.elapsed().as_micros() / PART2_BENCHES;
    assert_eq!(part2, 10219);

    format!(
        "Parser took {} µs\n\tP1: {} ({} ms)\n\tP2: {} ({} µs)",
        parser_time, part1, part1_time, part2, part2_time
    )
}

fn part_1(bags: &Bags) -> u32 {
    let mut open = VecDeque::new();
    let mut closed = HashSet::new();

    let first = SHINY_GOLD.to_string();
    open.push_front(&first);

    while open.len() > 0 {
        let val = open.pop_back().unwrap();
        closed.insert(val);

        bags.contents
            .iter()
            .filter(|kv| kv.1.contents.contains_key(val) && !closed.contains(kv.0))
            .for_each(|kv| open.push_front(&kv.0));
    }

    closed.len() as u32 - 1
}

#[derive(Default)]
struct BagTree {
    next: Option<Vec<(u32, BagTree)>>,
    count: u32,
}

impl BagTree {
    pub fn construct(bags: &Bags, key: String) -> BagTree {
        let bag = bags.contents.get(&key);

        match bag {
            Some(bag) => {
                let below = {
                    let children: Vec<(u32, BagTree)> = bag
                        .contents
                        .iter()
                        .map(|kv| (*kv.1, BagTree::construct(bags, kv.0.to_string())))
                        .collect();

                    (children.len() != 0).then_some(children)
                };

                BagTree {
                    next: below,
                    count: bag.contents.values().sum(),
                }
            }
            None => BagTree::default(),
        }
    }

    pub fn print_tree(&self) {
        if let Some(children) = &self.next {
            children.iter().for_each(|c| c.1.print_tree());
        }
    }

    pub fn part_2(&self) -> u32 {
        let mut sum = self.count;
        if let Some(children) = &self.next {
            for child in children {
                sum += child.1.part_2() * child.0;
            }
        }

        sum
    }
}
