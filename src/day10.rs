use std::{collections::VecDeque, fs, iter::Inspect};

use itertools::Itertools;

pub fn day10() {
    let input = fs::read_to_string("./input/day10/test-input.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day10/input.txt").expect("Couldn't load input");

    let instructions: VecDeque<Instruction> = input
        .lines()
        .filter_map(|line| Instruction::from_string(line))
        .collect();
    println!("instructions: {:?}", instructions);
    let mut cpu = CPU::new(instructions);
    let regxs: Vec<i32> = cpu
        .reg_xs()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, regx)| (i as i32 + 1) * regx)
        .collect();
    let sum: i32 = regxs.iter().sum();
    println!("regxs: {:?}, sum: {sum}", regxs);
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_string(str: &str) -> Option<Instruction> {
        if str.starts_with("noop") {
            Some(Instruction::Noop)
        } else if str.starts_with("addx") {
            let (_, operand) = str.split_once(" ")?;
            Some(Instruction::Addx(operand.parse().ok()?))
        } else {
            None
        }
    }

    fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

struct CPU {
    instructions: VecDeque<Instruction>,
    current: Option<Instruction>,
    cycle_counter: u32,
    last_fetch: u32,
    reg_x: i32,
}

impl CPU {
    fn new(mut instructions: VecDeque<Instruction>) -> CPU {
        let current = instructions.pop_front();
        CPU {
            instructions,
            current,
            cycle_counter: 0,
            last_fetch: 0,
            reg_x: 1,
        }
    }

    fn cycles_for_current(&self) -> u32 {
        self.cycle_counter - self.last_fetch
    }

    fn cycle(&mut self) {
        if let Some(current) = &self.current {
            if self.cycles_for_current() >= current.cycles() {
                self.execute();
                self.current = self.instructions.pop_front();
                self.last_fetch = self.cycle_counter
            }
            println!("cycle: {}, regx: {}", self.cycle_counter, self.reg_x);
            self.cycle_counter += 1;
        }
    }

    fn execute(&mut self) {
        if let Some(current) = &self.current {
            match current {
                Instruction::Addx(value) => self.reg_x += value,
                Instruction::Noop => (),
            }
            println!("exeucte: {:?}, new reg_x: {}", current, self.reg_x);
        }
    }

    fn reg_xs(&mut self) -> RegXs<'_> {
        RegXs { cpu: self }
    }
}

struct RegXs<'a> {
    cpu: &'a mut CPU,
}

impl<'a> Iterator for RegXs<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cpu.cycle();
        if let Some(_) = &self.cpu.current {
            Some(self.cpu.reg_x)
        } else {
            None
        }
    }
}
