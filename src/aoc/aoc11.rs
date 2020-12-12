use std::cell::RefCell;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

pub fn aoc_11(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let part1 = run_solver(&lines, update_state_p1);
    assert_eq!(2418, part1);
    let part2 = run_solver(&lines, update_state_p2);
    assert_eq!(2144, part2);

    // let sw = std::time::Instant::now();
    // for _ in 0..100 {
    // let _ = run_solver(&lines, update_state_p1);
    // }
    // let part1_time = sw.elapsed().as_millis() / 100;
    // println!("{}", part1_time);

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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum State {
    None,
    Floor,
    Empty,
    Occupied,
}

struct Grid {
    size: (usize, usize),
    content: Rc<RefCell<Vec<State>>>,
    swapchain: Rc<RefCell<Vec<State>>>,

    content1: Rc<RefCell<Vec<State>>>,
    content2: Rc<RefCell<Vec<State>>>,
    backbuffer: i8,
}

impl Grid {
    pub fn new(lines: &Vec<String>) -> Grid {
        let mut grid1 = Vec::new();
        let mut grid2 = Vec::new();
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
            .for_each(|s| {
                grid1.push(s);
                grid2.push(s);
            });

        let swapchain = Rc::new(RefCell::new(grid2));
        let grid = Rc::new(RefCell::new(grid1));

        Grid {
            content: grid.clone(),
            swapchain: swapchain.clone(),
            content1: grid.clone(),
            content2: swapchain.clone(),
            size: (lines[0].len(), lines.len()),
            backbuffer: 1,
        }
    }

    pub fn swap_contents(&mut self) {
        match self.backbuffer {
            1 => {
                assert_eq!(self.content, self.content1);
                assert_eq!(self.swapchain, self.content2);

                self.content = self.content2.clone();
                self.swapchain = self.content1.clone();
            }
            -1 => {
                assert_eq!(self.content, self.content2);
                assert_eq!(self.swapchain, self.content1);

                self.content = self.content1.clone();
                self.swapchain = self.content2.clone();
            }
            _ => (),
        }
        self.backbuffer *= -1;
    }
}

fn run_solver(lines: &Vec<String>, rule_handler: fn(&Grid, usize) -> State) -> u32 {
    let mut grid = Grid::new(lines);
    let content_len = grid.content.borrow().len();

    loop {
        let mut changed = false;
        for i in 0..content_len {
            let state = grid.content.borrow()[i];
            if state == State::Floor {
                continue;
            }

            let next_state = (rule_handler)(&grid, i);

            if state != next_state {
                grid.swapchain.borrow_mut()[i] = next_state;
                changed = true;
            }
        }

        grid.swap_contents();
        if !changed {
            break;
        }
    }

    let grid_content = grid.content.borrow_mut();
    grid_content
        .iter()
        .filter(|&&v| v == State::Occupied)
        .count() as u32
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

fn update_state_p1(grid: &Grid, index: usize) -> State {
    let state = grid.content.borrow()[index];

    let mut occupied = 0;
    for &offset in OFFSETS {
        let pos = add_pos(pos_from_index(index, grid.size), offset);
        if !pos_in_bounds(pos, grid.size) {
            continue;
        }
        let pos = index_from_pos(pos, grid.size);
        if grid.content.borrow()[pos] == State::Occupied {
            occupied += 1;
        }
    }

    match state {
        State::Occupied if occupied >= 4 => State::Empty,
        State::Empty if occupied == 0 => State::Occupied,
        _ => state,
    }
}

fn update_state_p2(grid: &Grid, index: usize) -> State {
    let state = grid.content.borrow()[index];

    let mut occupied = 0;
    for &offset in OFFSETS {
        let mut pos = add_pos(pos_from_index(index, grid.size), offset);
        while pos_in_bounds(pos, grid.size) {
            let index = index_from_pos(pos, grid.size);
            let grid_state = grid.content.borrow()[index];
            if grid_state == State::Occupied {
                occupied += 1;
            }
            if grid_state != State::Floor {
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
