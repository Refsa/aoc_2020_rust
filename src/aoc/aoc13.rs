use std::{cmp::Ordering, f32, i32};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};
use gcd::*;

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

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / a.gcd(b)
}

fn solve_part2(lines: &Vec<String>) -> u32 {
    let mut busses: Vec<(usize, usize)> = Vec::new();

    for (i, c) in lines[1].split(',').enumerate() {
        if c != "x" {
            busses.push((c.parse::<usize>().unwrap(), i));
        }
    }

    let first = busses.first().unwrap();
    let last = busses.last().unwrap();

    let step = first.0;
    let mut timestamp = step;
    
    let mut l = 1;
    let mut m = 1;
    for (val, i) in busses {
        l = lcm(l, val + i);
        m *= val;
    }

    l = l % m;

    println!("{} - {}", l, l % 3417);

    // println!("{}", timestamp);

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
