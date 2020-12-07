use crate::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

const SHINY_GOLD: &str = "shiny gold";

const PART1_BENCHES: u128 = 100;
const PART2_BENCHES: u128 = 100;

#[derive(Debug, Default, Clone)]
struct Bag {
    contents: HashMap<String, u32>,
}

#[derive(Default, Debug)]
struct Bags {
    contents: HashMap<String, Bag>,
}

fn digest_content(line: &str) -> (String, u32) {
    let inner = line.trim();
    let split = inner.rmatch_indices("bag").nth(0).unwrap();
    let inner = inner.split_at(split.0).0;

    let (count, color) = inner.split_at(1);
    let color = color.trim().to_owned();
    let count: u32 = count.parse().unwrap();

    (color, count)
}

pub fn aoc_7(reader: BufReader<File>) -> String {
    let mut bags: Bags = Default::default();

    reader
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .for_each(|l| {
            let content: Vec<&str> = l.split("bags contain").collect();

            let mut bag = Bag::default();
            if !content[1].contains("no other bags") {
                for (color, count) in content[1].split(',').map(|i| digest_content(i)) {
                    bag.contents.insert(color, count);
                }
            }
            bags.contents.insert(content[0].trim().to_owned(), bag);
        });

    let sw = std::time::Instant::now();
    let mut part1 = 0u32;
    for i in 0..PART1_BENCHES {
        part1 = part_1(&bags);
    }
    println!("Part 1 Time: {} ms avg", sw.elapsed().as_millis() / PART1_BENCHES);

    let sw = std::time::Instant::now();
    let mut part2 = 0u32;
    for i in 0..PART2_BENCHES {
        let tree = BagTree::construct(&bags, SHINY_GOLD.to_string());
        let part2 = tree.part_2();
    }
    println!("Part 1 Time: {} µs avg", sw.elapsed().as_micros() / PART2_BENCHES);


    format!("P1: {}\n\tP2: {}", part1, part2)
}

fn part_1(bags: &Bags) -> u32 {
    let mut count = 0;
    let mut open = Vec::new();
    let mut closed = HashSet::new();
    open.push(SHINY_GOLD);
    loop {
        let val = open.pop().unwrap().to_owned();
        closed.insert(val.to_owned());

        for (k, v) in &bags.contents {
            if v.contents.contains_key(&val) && !closed.contains(k) {
                count += 1;
                open.push(k);
            }
        }

        if open.len() == 0 {
            break;
        }
    }
    count
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
