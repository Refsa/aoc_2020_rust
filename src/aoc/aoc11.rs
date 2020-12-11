use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    thread::current,
};

#[derive(Eq, PartialEq, Copy, Clone)]
enum States {
    None,
    Empty,
    Floor,
    Occupied,
}

pub fn aoc_11(reader: BufReader<File>) -> String {
    let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let part1 = part_1(&lines);

    format!("{}", part1)
}

fn generate_grid(lines: &Vec<String>) -> Vec<Vec<States>> {
    let mut grid = Vec::new();

    for line in lines {
        let line_states: Vec<States> = line
            .chars()
            .map(|c| match c {
                '.' => States::Floor,
                'L' => States::Empty,
                _ => States::None,
            })
            .collect();
        grid.push(line_states);
    }

    grid
}

fn part_2(lines: &Vec<String>) -> u32 {
    let mut grid = generate_grid(lines);

    let mut state_changed = true;
    while state_changed {
        let mut changed_cells = Vec::new();
        state_changed = false;
        for x in 0..grid[0].len() {
            for y in 0..grid.len() {
                let state = grid[y][x];
                let next_state = check_state_p1(&grid, (x, y));

                if state != next_state {
                    changed_cells.push((next_state, x, y));
                    state_changed = true;
                }
            }
        }

        for cc in changed_cells {
            grid[cc.2][cc.1] = cc.0;
        }
    }

    0
}

fn part_1(lines: &Vec<String>) -> u32 {
    let mut grid = generate_grid(lines);

    let mut state_changed = true;
    while state_changed {
        let mut changed_cells = Vec::new();
        state_changed = false;
        for x in 0..grid[0].len() {
            for y in 0..grid.len() {
                let state = grid[y][x];
                let next_state = check_state_p1(&grid, (x, y));

                if state != next_state {
                    changed_cells.push((next_state, x, y));
                    state_changed = true;
                }
            }
        }

        for cc in changed_cells {
            grid[cc.2][cc.1] = cc.0;
        }
    }

    let mut occupied_count = 0;
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == States::Occupied {
                occupied_count += 1;
            }
        }
    }

    occupied_count
}

fn check_state_p1(grid: &Vec<Vec<States>>, pos: (usize, usize)) -> States {
    let self_state = grid[pos.1][pos.0];
    if self_state == States::Floor || self_state == States::None {
        return self_state;
    }

    let mut occupied = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            let px = pos.0 as i32 + x;
            let py = pos.1 as i32 + y;
            if x == 0 && y == 0
                || px < 0
                || py < 0
                || px >= grid[0].len() as i32
                || py >= grid.len() as i32
            {
                continue;
            }
            let px = px as usize;
            let py = py as usize;

            if grid[py][px] == States::Occupied {
                occupied += 1;
            }
        }
    }

    match self_state {
        States::Occupied => {
            if occupied >= 4 {
                States::Empty
            } else {
                States::Occupied
            }
        }
        States::Empty => {
            if occupied == 0 {
                States::Occupied
            } else {
                States::Empty
            }
        }
        _ => self_state,
    }
}
