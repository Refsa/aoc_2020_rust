use std::fs::File;
use std::io::prelude::*;
use std::{io::BufReader};

const TREE_SQUARE: char = '#';

pub fn aoc_3(reader: BufReader<File>) -> String{
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let width = lines[0].len();
    let checks: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut total = 1usize;
    
    for check in checks {
        let mut tree_count = 0;
        let mut index_x = 0;
        let mut index_y = 0;
        loop {
            let square = lines[index_y].chars().nth(index_x).unwrap();

            if square == TREE_SQUARE {
                tree_count += 1;
            }

            index_x = (index_x + check.0) % width;
            index_y += check.1;
            if index_y >= lines.len() {
                break;
            }
        }
        total *= tree_count;
    }

    format!("total: {}", total)
}
