use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn aoc_8(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let program = Program::new(&lines);

    let part1 = part_1_buckets(&program);
    assert_eq!(part1, 1810);
    let sw = std::time::Instant::now();
    for i in 0..10000 {
        let part1 = part_1_buckets(&program);
    }
    let p1_elapsed = sw.elapsed().as_micros() / 10000;

    let part2 = part_2(&program);
    assert_eq!(part2, 969);
    let sw = std::time::Instant::now();
    for i in 0..10000 {
        let part2 = part_2(&program);
    }
    let p2_elapsed = sw.elapsed().as_micros() / 10000;

    format!(
        "Part1 (~{} µs): {}\n\tPart2 (~{} µs): {}",
        p1_elapsed, part1, p2_elapsed, part2
    )
}

// Since the problem space is small a simple bucket can track our progress.
// This gives us control over where the memory is allocated (stack/heap)
fn part_1_buckets(program: &Program) -> i64 {
    let mut state = State::default();

    // Stack allocated (~6µs) but at a cost of space
    let mut ran_ops = [0u8; 2048];

    // Heap allocated (~14µs) but we only allocate what we need
    // let mut ran_ops = vec![0u8; program.code.len()];

    loop {
        let cl = state.current_line;
        program.run_line(cl, &mut state);

        ran_ops[cl] = 1;

        if ran_ops[state.current_line] != 0 {
            break;
        }
    }

    state.accumulator
}

// HashSet (~220µs) requires no knowledge of space ahead of time at a huge time cost
fn part_1_hashset(program: &Program) -> i64 {
    let mut state = State::default();
    let mut ran_ops = HashSet::new();

    loop {
        let cl = state.current_line;
        program.run_line(cl, &mut state);

        if !ran_ops.insert(state.current_line) {
            break;
        }
    }

    state.accumulator
}

fn part_2(program: &Program) -> i64 {
    let mut state = State::default();
    let mut ran_ops = [0u8; 1024];
    let mut op_seq = [0usize; 512];
    let mut op_seq_count = 0;
    let mut looped_at: &usize = &0usize;

    loop {
        let cl = state.current_line;
        op_seq[op_seq_count] = cl;
        op_seq_count += 1;

        program.run_line(cl, &mut state);

        ran_ops[cl] = 1;
        if ran_ops[state.current_line] != 0 {
            looped_at = &state.current_line;
            break;
        }
    }

    op_seq
        .iter()
        .skip_while(|v| *v != looped_at)
        .skip(1)
        .take_while(|v| *v != looped_at)
        .filter(|v| program.code[**v].op_code != OpCode::ACC)
        .filter_map(|rt| {
            if let Some(acc) = part_2_rerunner(&program, *rt) {
                Some(acc)
            } else {
                None
            }
        })
        .nth(0)
        .unwrap_or_default()
}

fn part_2_rerunner(program: &Program, replace: usize) -> Option<i64> {
    let mut state = State::default();
    let mut ran_ops = [0u8; 1024];

    while !program.is_eop(state.current_line) {
        let cl = state.current_line;

        if cl == replace {
            let op = match program.code[cl].op_code {
                OpCode::JMP => OpCode::NOP,
                OpCode::NOP => OpCode::JMP,
                _ => OpCode::UNKNOWN,
            };

            program.run_line_with_op(cl, op, &mut state);
        } else {
            program.run_line(cl, &mut state);
        }

        ran_ops[cl] = 1;
        if ran_ops[state.current_line] != 0 {
            return None;
        }
    }
    
    Some(state.accumulator)
}

struct Program {
    code: Vec<Op>,
}

impl Program {
    pub fn new(lines: &Vec<String>) -> Program {
        let code = lines
            .iter()
            .map(|l| {
                let (op, num) = Self::parse_line(l);
                Op {
                    op_code: op.into(),
                    value: num,
                }
            })
            .collect();

        Program { code: code }
    }

    fn parse_line(line: &String) -> (&str, i64) {
        let (op, num) = line.split_once(' ').unwrap();
        let num: i64 = num.parse().unwrap();

        (op, num)
    }

    pub fn run_line(&self, line: usize, state: &mut State) {
        let op = &self.code[line];
        self.run_op(op, &op.op_code, state);
    }

    pub fn run_line_with_op(&self, line: usize, op_code: OpCode, state: &mut State) {
        let op = &self.code[line];
        self.run_op(op, &op_code, state);
    }

    fn run_op(&self, op: &Op, op_code: &OpCode, state: &mut State) {
        match op_code {
            OpCode::JMP => state.current_line = (state.current_line as i64 + op.value) as usize,
            OpCode::ACC => {
                state.accumulator += op.value;
                state.current_line += 1;
            }
            OpCode::NOP => {
                state.current_line += 1;
            }
            OpCode::UNKNOWN => (),
        }
    }

    pub fn is_eop(&self, line: usize) -> bool {
        line >= self.code.len()
    }
}

struct Op {
    op_code: OpCode,
    value: i64,
}

#[derive(Default, Copy, Clone)]
struct State {
    accumulator: i64,
    current_line: usize,
}

#[derive(Eq, PartialEq)]
#[repr(u32)]
enum OpCode {
    UNKNOWN,
    JMP,
    ACC,
    NOP,
}

impl From<&str> for OpCode {
    fn from(val: &str) -> Self {
        match val {
            "jmp" => OpCode::JMP,
            "acc" => OpCode::ACC,
            "nop" => OpCode::NOP,
            _ => OpCode::UNKNOWN,
        }
    }
}
