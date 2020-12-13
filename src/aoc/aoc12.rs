use std::cell::RefCell;
use std::{f32, i32};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

pub fn aoc_12(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Benching
    let sw = std::time::Instant::now();
    for _ in 0..10000 {
        let _ = compact_solution_1(&lines);
    }
    let part1_time = sw.elapsed().as_nanos() / 10000;

    let sw = std::time::Instant::now();
    for _ in 0..10000 {
        let _ = compact_solution_2(&lines);
    }
    let part2_time = sw.elapsed().as_nanos() / 10000;

    // // Solving
    let part1 = compact_solution_1(&lines);
    assert_eq!(904, part1);
    let part2 = compact_solution_2(&lines);
    assert_eq!(18747, part2);

    format!(
        "P1 (~{}ns): {}\n\tP2 (~{}ns): {}",
        part1_time, part1, part2_time, part2
    )
}

// #### Compact solution ####
#[derive(Copy, Clone)]
enum CompactOpCode {
    None,
    Heading((i32, i32)),
    Direction(i32, i32),
}

const HEADINGS: &[(i32, i32)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
const ROTATIONS: &[(i32, i32)] = &[(-1, 1), (1, -1), (1, -1), (-1, 1)];

fn parse_compact(op_val: (&str, &str)) -> CompactOpCode {
    let (op, val) = op_val;
    let val: i32 = val.parse().unwrap();

    match op {
        "N" => CompactOpCode::Heading((0, val)),
        "E" => CompactOpCode::Heading((val, 0)),
        "S" => CompactOpCode::Heading((0, -val)),
        "W" => CompactOpCode::Heading((-val, 0)),
        "L" => CompactOpCode::Direction(1, -(val / 90)),
        "R" => CompactOpCode::Direction(2, val / 90),
        "F" => CompactOpCode::Direction(0, val),
        _ => CompactOpCode::None,
    }
}

fn swap(target: &mut (i32, i32), sign: &(i32, i32)) {
    let temp = target.0;
    target.0 = target.1 * sign.0;
    target.1 = temp * sign.1;
}

fn compact_solution_1(lines: &Vec<String>) -> i32 {
    let (_, (part1_x, part1_y)) =
        lines
            .iter()
            .map(|l| parse_compact(l.split_at(1)))
            .fold((1i32, (0, 0)), |mut a, op| {
                match op {
                    CompactOpCode::None => (),
                    CompactOpCode::Heading((x, y)) => {
                        a.1 .0 += x;
                        a.1 .1 += y;
                    }
                    CompactOpCode::Direction(dir, val) => match dir {
                        0 => {
                            a.1 .0 += HEADINGS[a.0 as usize].0 * val;
                            a.1 .1 += HEADINGS[a.0 as usize].1 * val;
                        }
                        1 | 2 => {
                            a.0 += val;
                            if a.0 > 3 {
                                a.0 -= 4;
                            } else if a.0 < 0 {
                                a.0 += 4;
                            }
                        }
                        _ => (),
                    },
                }
                a
            });

    part1_x.abs() + part1_y.abs()
}

fn compact_solution_2(lines: &Vec<String>) -> i32 {
    let (_, (part2_x, part2_y)) =
        lines
            .iter()
            .map(|l| parse_compact(l.split_at(1)))
            .fold(((10, 1), (0, 0)), |mut a, op| {
                match op {
                    CompactOpCode::None => (),
                    CompactOpCode::Heading((x, y)) => {
                        a.0 .0 += x;
                        a.0 .1 += y;
                    }
                    CompactOpCode::Direction(dir, val) => match dir {
                        0 => {
                            a.1 .0 += a.0 .0 * val;
                            a.1 .1 += a.0 .1 * val;
                        }
                        1 | 2 if val.abs() == 2 => {
                            a.0 .0 = -a.0 .0;
                            a.0 .1 = -a.0 .1;
                        }
                        1 | 2 => swap(
                            &mut a.0,
                            &ROTATIONS[(dir * (val.abs() + 1) / 2 - 1) as usize],
                        ),
                        _ => (),
                    },
                }
                a
            });

    part2_x.abs() + part2_y.abs()
}

// #### Longer Solution but more clear ####
fn long_solution(lines: &Vec<String>) -> (i32, i32) {
    // // Parsing
    let ops: Vec<OpCode> = lines
        .iter()
        .map(|v| {
            let (op, val) = v.split_at(1);
            get_opcode(op, val)
        })
        .collect();

    (solve_p1(&ops), solve_p2(&ops))
}

fn solve_p2(ops: &Vec<OpCode>) -> i32 {
    let mut boat = Boat {
        heading: Heading::E(0),
        travel_distance: (0, 0),
    };

    let mut waypoint = (10, 1);

    for op in ops {
        handle_op_p2(&op, &mut boat, &mut waypoint);
    }

    boat.travel_distance.0.abs() + boat.travel_distance.1.abs()
}

fn handle_op_p2(op: &OpCode, boat: &mut Boat, waypoint: &mut (i32, i32)) {
    match op {
        OpCode::None => {}
        OpCode::Heading(h) => match h {
            Heading::N(val) => waypoint.1 += val,
            Heading::S(val) => waypoint.1 -= val,
            Heading::E(val) => waypoint.0 += val,
            Heading::W(val) => waypoint.0 -= val,
        },
        OpCode::Direction(d) => match d {
            Direction::L(val) => match val {
                90 => {
                    let temp = waypoint.0;
                    waypoint.0 = -waypoint.1;
                    waypoint.1 = temp;
                }
                180 => {
                    waypoint.0 = -waypoint.0;
                    waypoint.1 = -waypoint.1;
                }
                270 => {
                    let temp = waypoint.0;
                    waypoint.0 = waypoint.1;
                    waypoint.1 = -temp;
                }
                _ => (),
            },
            Direction::R(val) => match val {
                90 => {
                    let temp = waypoint.0;
                    waypoint.0 = waypoint.1;
                    waypoint.1 = -temp;
                }
                180 => {
                    waypoint.0 = -waypoint.0;
                    waypoint.1 = -waypoint.1;
                }
                270 => {
                    let temp = waypoint.0;
                    waypoint.0 = -waypoint.1;
                    waypoint.1 = temp;
                }
                _ => (),
            },
            Direction::F(val) => {
                boat.travel_distance.0 += waypoint.0 * val;
                boat.travel_distance.1 += waypoint.1 * val;
            }
        },
    }
}

fn solve_p1(ops: &Vec<OpCode>) -> i32 {
    let mut boat = Boat {
        heading: Heading::E(0),
        travel_distance: (0, 0),
    };

    for op in ops {
        handle_op_p1(&op, &mut boat);
    }

    let part1 = boat.travel_distance.0.abs() + boat.travel_distance.1.abs();

    part1
}

fn handle_op_p1(op: &OpCode, boat: &mut Boat) {
    match op {
        OpCode::None => {}
        OpCode::Heading(h) => match h {
            Heading::N(val) => {
                boat.travel_distance.0 += val;
            }
            Heading::S(val) => {
                boat.travel_distance.0 -= val;
            }
            Heading::E(val) => {
                boat.travel_distance.1 += val;
            }
            Heading::W(val) => {
                boat.travel_distance.1 -= val;
            }
        },
        OpCode::Direction(d) => match d {
            Direction::L(val) => {
                change_heading_p1(d, boat);
            }
            Direction::R(val) => {
                change_heading_p1(d, boat);
            }
            Direction::F(val) => {
                handle_op_p1(&OpCode::Heading(make_heading(&boat.heading, *val)), boat);
            }
        },
    }
}

fn change_heading_p1(direction: &Direction, boat: &mut Boat) {
    let change = match direction {
        Direction::L(val) => -direction_to_int(*val),
        Direction::R(val) => direction_to_int(*val),
        _ => 0,
    };

    let current_heading = match heading_to_int(&boat.heading) + change {
        h if h < 0 => h + 4,
        h if h > 3 => h - 4,
        h => h,
    };

    boat.heading = int_to_heading(current_heading);
}

#[derive(Debug)]
enum Heading {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
}

#[derive(Debug)]
enum Direction {
    L(i32),
    R(i32),
    F(i32),
}

#[derive(Debug)]
enum OpCode {
    None,
    Heading(Heading),
    Direction(Direction),
}

fn get_opcode(op: &str, val: &str) -> OpCode {
    match op {
        "N" => OpCode::Heading(Heading::N(val.parse().unwrap())),
        "E" => OpCode::Heading(Heading::E(val.parse().unwrap())),
        "S" => OpCode::Heading(Heading::S(val.parse().unwrap())),
        "W" => OpCode::Heading(Heading::W(val.parse().unwrap())),
        "L" => OpCode::Direction(Direction::L(val.parse().unwrap())),
        "R" => OpCode::Direction(Direction::R(val.parse().unwrap())),
        "F" => OpCode::Direction(Direction::F(val.parse().unwrap())),
        _ => OpCode::None,
    }
}

#[derive(Debug)]
struct Boat {
    heading: Heading,
    travel_distance: (i32, i32),
}

fn heading_to_int(heading: &Heading) -> i32 {
    match heading {
        Heading::N(_) => 0,
        Heading::S(_) => 2,
        Heading::E(_) => 1,
        Heading::W(_) => 3,
    }
}

fn int_to_heading(heading: i32) -> Heading {
    match heading {
        0 => Heading::N(0),
        1 => Heading::E(0),
        2 => Heading::S(0),
        3 => Heading::W(0),
        _ => panic!(format!("wattafak {}", heading)),
    }
}

fn direction_to_int(direction: i32) -> i32 {
    match direction {
        0 => 0,
        90 => 1,
        180 => 2,
        270 => 3,
        _ => 0,
    }
}

fn make_heading(heading: &Heading, val: i32) -> Heading {
    match heading {
        Heading::N(_) => Heading::N(val),
        Heading::S(_) => Heading::S(val),
        Heading::E(_) => Heading::E(val),
        Heading::W(_) => Heading::W(val),
    }
}
