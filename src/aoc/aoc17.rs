use std::{
    collections::{HashMap, HashSet},
    usize,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum State {
    Active,
    Inactive,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

struct Node {
    pos: Vec3,
    state: (State, State),
    neighbours: Vec<Vec3>,
}

pub fn aoc_17(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut grid = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vec3 {
                x: x as i32,
                y: y as i32,
                z: 0,
            };

            let node = Node {
                pos: pos.clone(),
                state: match c {
                    '#' => (State::Active, State::Active),
                    _ => (State::Inactive, State::Inactive),
                },
                neighbours: Vec::new(),
            };

            grid.insert(pos, node);
        }
    }

    for _ in 0..6 {
        for (p, n) in &mut grid {
            tick_node(&mut grid, p);
        }
    }

    format!("")
}

fn tick_node(grid: &mut HashMap<Vec3, Node>, pos: &Vec3) {
    let mut node = grid.get_mut(pos).unwrap();

    if node.neighbours.len() == 0 {
        for x in -1..1 {
            for y in -1..1 {
                for z in -1..1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }

                    let pos = Vec3{x: x, y: y, z: z};
                    if grid.contains_key(&pos) {
                        node.neighbours.push(pos);
                    }
                }
            }
        }

    }

    
}
