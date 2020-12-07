use crate::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

const SHINY_GOLD: &str = "shiny gold";

#[derive(Debug, Default, Clone)]
struct Bag {
    contents: HashMap<String, u32>,
}

impl Bag {
    pub fn new() -> Self {
        Bag {
            contents: HashMap::new(),
        }
    }
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

            let mut bag = Bag::new();
            if !content[1].contains("no other bags") {
                for (color, count) in content[1].split(',').map(|i| digest_content(i)) {
                    bag.contents.insert(color, count);
                }
            }
            bags.contents.insert(content[0].trim().to_owned(), bag);
        });

    let tree = BagTree::construct(&bags, SHINY_GOLD.to_string());

    format!("{}\n\t{}", part_1(&bags), tree.part_2())
}

fn part_1(bags: &Bags) -> u32 {
    let mut count = 0;
    let mut outer = Vec::new();
    let mut searched = HashSet::new();
    outer.push(SHINY_GOLD);
    loop {
        let val = outer.pop().unwrap().to_owned();
        searched.insert(val.to_owned());

        for (k, v) in &bags.contents {
            if v.contents.contains_key(&val) && !searched.contains(k) {
                count += 1;
                outer.push(k);
            }
        }

        if outer.len() == 0 {
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
