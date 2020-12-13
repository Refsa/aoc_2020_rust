use std::{cmp::Ordering, f32, i32};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

fn frac(val: f32) -> f32 {
    val - val as u32 as f32
}

pub fn aoc_13(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let part1 = solve_part1(&lines);
    // assert_eq!(161, part1);
    let part2 = solve_part2(&lines);

    format!("Part1: {}\n\tPart2: {}", part1, part2)
}

fn solve_part2(lines: &Vec<String>) -> u32 {
    let busses: Vec<u32> = lines[1]
        .split(',')
        .map(|v| {
            if v == "x" {
                0
            } else {
                v.parse::<u32>().unwrap()
            }
        })
        .map(|v| v)
        .collect();

    println!("{:?} - {}", busses, busses.len());
    let zeros = busses.iter().filter(|&&b| b == 0).count();

    let step = busses[0] as usize;
    let mut timestamp = 100_000_000_000_000 - (100_000_000_000_000 % step);
    let mut hits = zeros;

    // BusID % (29 * Timestamp) = Index

    while hits != busses.len() {
        hits = zeros;
        timestamp += step;

        if timestamp % 10_000_000 == 0 {
            println!("{}", timestamp);
        }

        for (i, &t) in busses.iter().enumerate() {
            if t == 0 {
                continue;
            }

            let mut at = timestamp % t as usize;
            if at != 0 {
                at = t as usize - at;
            }

            if at == i {
                hits += 1;
            }
        }
    }

    println!("{}", timestamp);

    0
}

fn solve_part1(lines: &Vec<String>) -> u32 {
    let dtime: u32 = lines[0].parse().unwrap();
    let mut busses: Vec<u32> = lines[1]
        .split(',')
        .filter_map(|v| {
            if v == "x" {
                None
            } else {
                Some(v.parse::<u32>().unwrap())
            }
        })
        .map(|v| v)
        .collect();
    busses.sort();
    let largest = *busses.last().unwrap();
    let busses = busses
        .iter()
        .map(|&v| (v, largest as f32 / v as f32))
        .map(|v| (v.0, v.1, dtime as f32 / v.0 as f32));

    let bus = busses
        .max_by(|x, y| {
            if frac(y.2) == 0f32 {
                return Ordering::Less;
            }

            let diff = frac(x.2) * x.1 - frac(y.2) * y.1;
            if diff > 0f32 {
                Ordering::Greater
            } else if diff == 0f32 {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        })
        .unwrap();

    let arrival_time = (bus.0 as f32 * (1f32 - frac(bus.2))) as u32;
    arrival_time * bus.0
}
