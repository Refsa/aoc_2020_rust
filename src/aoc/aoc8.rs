use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn aoc_8(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let program = Program::new(&lines);

    let part1 = part_1_buckets(&program);
    if false {
        let sw = std::time::Instant::now();
        for i in 0..10000 {
            let part1 = part_1_buckets(&program);
        }
        let elapsed = sw.elapsed().as_micros() / 10000;
        println!("Part 1 Buckets Time: {} µs", elapsed);
        assert_eq!(part1, 1810);

        let sw = std::time::Instant::now();
        for i in 0..10000 {
            let part1 = part_1_hashset(&program);
        }
        let elapsed = sw.elapsed().as_micros() / 10000;
        println!("Part 1 Hashset Time: {} µs", elapsed);
        assert_eq!(part1, 1810);
    }

    let part2 = part_2(&program);
    if true {
        let sw = std::time::Instant::now();
        for i in 0..1000 {
            let part2 = part_2(&program);
        }
        let elapsed = sw.elapsed().as_micros() / 1000;
        println!("Part 2 Time: {} µs", elapsed);
    }

    format!("Part1: {}\n\tPart2: {}", part1, part2)
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
    let mut ran_ops = [0u8; 2048];
    let mut op_seq = Vec::new();

    loop {
        let cl = state.current_line;
        op_seq.push(cl);

        program.run_line(cl, &mut state);

        ran_ops[cl] = 1;
        if ran_ops[state.current_line] != 0 {
            op_seq.push(state.current_line);
            break;
        }
    }

    let (mut s, mut e) = (0, 0);
    for i in 0..op_seq.len() {
        for j in i + 1..op_seq.len() {
            if op_seq[i] == op_seq[j] {
                s = i + 1;
                e = j;
            }
        }
    }

    let mut accumulator = 0;

    for rt in op_seq
        .iter()
        .skip(s)
        .take(e - s)
        .filter(|o| program.code[**o].op_code != OpCode::ACC)
    {
        let mut state = State::default();
        let mut ran_ops = [0u8; 2048];
        let mut success = true;

        loop {
            let cl = state.current_line;

            if cl == *rt {
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
                success = false;
                break;
            }
            if program.is_eop(state.current_line) {
                break;
            }
        }

        if success {
            accumulator = state.accumulator;
            break;
        }
    }

    accumulator
}

struct Program {
    code: Vec<Op>,
}

impl Program {
    pub fn new(lines: &Vec<String>) -> Program {
        let mut code = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let (op, num) = Self::parse_line(l);

            code.push(Op {
                line: i as u64,
                op_code: op.into(),
                value: num,
            });
        }

        Program { code: code }
    }

    fn parse_line(line: &String) -> (&str, i64) {
        let (op, offset) = line.split_at(3);
        let mut offset = offset.chars();
        let sign = if offset.nth(0).unwrap() == '+' { -1 } else { 1 };
        let num: i64 = offset.as_str().parse().unwrap();
        let num = num * sign;

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
    line: u64,
    op_code: OpCode,
    value: i64,
}

#[derive(Default, Copy, Clone)]
struct State {
    accumulator: i64,
    current_line: usize,
}

#[derive(Eq, PartialEq)]
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
