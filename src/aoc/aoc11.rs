use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    thread::current,
};

pub fn aoc_11(reader: BufReader<File>) -> String {
    let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let part1 = part_1(&lines);
    assert_eq!(2418, part1);
    let part2 = part_2(&lines);

    format!("P1: {}\n\tP2: {}", part1, part2)
}

const OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Eq, PartialEq, Copy, Clone)]
enum State {
    None,
    Floor,
    Empty,
    Occupied,
}

impl Into<char> for State {
    fn into(self) -> char {
        match self {
            State::None => ' ',
            State::Floor => '.',
            State::Empty => 'L',
            State::Occupied => '#',
        }
    }
}

struct Grid {
    size: (usize, usize),
    content: Vec<State>,
}

impl Grid {
    pub fn new(lines: &Vec<String>) -> Grid {
        let mut grid = Vec::new();

        lines
            .iter()
            .map(|l| l.chars())
            .flatten()
            .map(|c| match c {
                '.' => State::Floor,
                'L' => State::Empty,
                '#' => State::Occupied,
                _ => State::None,
            })
            .for_each(|s| grid.push(s));

        Grid {
            content: grid,
            size: (lines[0].len(), lines.len()),
        }
    }

    pub fn print_grid(&self) {
        for i in 1..=self.content.len() {
            let c: char = self.content[i - 1].into();
            print!("{}", c);
            if i % self.size.0 == 0 {
                println!("");
            }
        }
    }
}

fn pos_from_index(index: usize, size: (usize, usize)) -> (i32, i32) {
    ((index % size.0) as i32, (index / (size.1 - 2)) as i32)
}
fn add_pos(pos1: (i32, i32), pos2: (i32, i32)) -> (i32, i32) {
    (pos1.0 + pos2.0, pos1.1 + pos2.1)
}
fn index_from_pos(pos: (i32, i32), size: (usize, usize)) -> usize {
    (pos.1 * size.0 as i32 + pos.0) as usize
}
fn pos_in_bounds(pos: (i32, i32), size: (usize, usize)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < size.0 as i32 && pos.1 < size.1 as i32
}

fn run_solver(lines: &Vec<String>, rule_handler: fn(&Grid, usize) -> State) -> u32 {
    let mut grid = Grid::new(lines);

    loop {
        let mut update_cells = Vec::new();

        for i in 0..grid.content.len() {
            let state = grid.content[i];
            if state == State::Floor {
                continue;
            }

            let next_state = (rule_handler)(&grid, i);

            if state != next_state {
                update_cells.push((i, next_state));
            }
        }

        if update_cells.len() == 0 {
            break;
        }

        for (i, s) in update_cells {
            grid.content[i] = s;
        }
    }

    grid.content
        .iter()
        .filter(|&&v| v == State::Occupied)
        .count() as u32
}

fn part_2(lines: &Vec<String>) -> u32 {
    let mut grid = Grid::new(lines);

    loop {
        let mut update_cells = Vec::new();

        for i in 0..grid.content.len() {
            let state = grid.content[i];
            if state == State::Floor {
                continue;
            }

            let next_state = update_state_p2(&grid, i);

            if state != next_state {
                update_cells.push((i, next_state));
            }
        }

        if update_cells.len() == 0 {
            break;
        }

        for (i, s) in update_cells {
            grid.content[i] = s;
        }
    }

    grid.content
        .iter()
        .filter(|&&v| v == State::Occupied)
        .count() as u32
}

fn update_state_p2(grid: &Grid, index: usize) -> State {
    let state = grid.content[index];

    let mut occupied = 0;
    for &offset in OFFSETS {
        let mut pos = add_pos(pos_from_index(index, grid.size), offset);
        while pos_in_bounds(pos, grid.size) {
            let index = index_from_pos(pos, grid.size);
            if grid.content[index] == State::Occupied {
                occupied += 1;
            }
            if grid.content[index] != State::Floor {
                break;
            }
            pos = add_pos(pos, offset);
        }
    }

    match state {
        State::Occupied if occupied >= 5 => State::Empty,
        State::Empty if occupied == 0 => State::Occupied,
        _ => state,
    }
}

fn part_1(lines: &Vec<String>) -> u32 {
    let mut grid = Grid::new(lines);

    println!("{:?}", pos_from_index(96, grid.size));

    loop {
        let mut update_cells = Vec::new();

        for i in 0..grid.content.len() {
            let state = grid.content[i];
            if state == State::Floor {
                continue;
            }

            let next_state = update_state(&grid, i);

            if state != next_state {
                update_cells.push((i, next_state));
            }
        }

        if update_cells.len() == 0 {
            break;
        }

        for (i, s) in update_cells {
            grid.content[i] = s;
        }
    }

    grid.content
        .iter()
        .filter(|&&v| v == State::Occupied)
        .count() as u32
}

fn update_state(grid: &Grid, index: usize) -> State {
    let state = grid.content[index];

    let mut occupied = 0;
    for &offset in OFFSETS {
        let pos = add_pos(pos_from_index(index, grid.size), offset);
        if !pos_in_bounds(pos, grid.size) {
            continue;
        }
        let pos = index_from_pos(pos, grid.size);
        if grid.content[pos] == State::Occupied {
            occupied += 1;
        }
    }

    match state {
        State::Occupied if occupied >= 4 => State::Empty,
        State::Empty if occupied == 0 => State::Occupied,
        _ => state,
    }
}
