use itertools::Itertools;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::fs;

pub fn day5() {
    let stack = fs::read_to_string("./input/day5/stack-test.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day5/input-test.txt").expect("Couldn't load input");
    let stack = fs::read_to_string("./input/day5/stack.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day5/input.txt").expect("Couldn't load input");

    let mut ship = Ship::parse(&stack);
    println!("ship: {:?}", ship);
    let moves: Vec<MoveCmd> = input.lines().map(MoveCmd::parse_line).collect();
    println!("moves: {:?}", moves);
    for cmd in moves {
        ship.apply_move_9001(&cmd)
    }
    println!("top: {}", ship.get_top_crates());
}

#[derive(Debug)]
struct MoveCmd {
    from: usize,
    to: usize,
    times: u32,
}

impl MoveCmd {
    fn parse_line(line: &str) -> MoveCmd {
        let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = regex.captures(line).unwrap();
        let times: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let to: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        MoveCmd { from, to, times }
    }
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Vec<char>>,
}

impl Ship {
    fn new() -> Ship {
        Ship { stacks: Vec::new() }
    }

    fn get_stack(&self, x: usize) -> &Vec<char> {
        return &self.stacks[x];
    }

    fn get_stack_mut(&mut self, x: usize) -> &mut Vec<char> {
        while self.stacks.len() <= x {
            self.stacks.push(Vec::new())
        }
        return &mut self.stacks[x];
    }

    fn add(&mut self, x: usize, container: &Option<char>) {
        if let Some(container) = container {
            self.get_stack_mut(x).insert(0, *container)
        }
    }

    fn apply_move(&mut self, cmd: &MoveCmd) {
        for _ in 0..cmd.times {
            let from_stack = self.get_stack_mut(cmd.from - 1);
            let val = from_stack.pop().unwrap();

            let to_stack = self.get_stack_mut(cmd.to - 1);
            to_stack.push(val)
        }
    }

    fn apply_move_9001(&mut self, cmd: &MoveCmd) {
        let from_stack = self.get_stack_mut(cmd.from - 1);
        let move_stack = from_stack.split_off(from_stack.len() - cmd.times as usize);
        let to_stack = self.get_stack_mut(cmd.to - 1);
        to_stack.extend(move_stack);
    }

    fn get_top_crates(&self) -> String {
        let mut result = String::new();
        for stack in self.stacks.iter() {
            if let Some(top) = stack.last() {
                result.push(*top)
            }
        }
        result
    }
}

impl Ship {
    fn parse(input: &str) -> Ship {
        fn parse_line(line: &str) -> Vec<Option<char>> {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|mut group| {
                    let group: String = group.collect();
                    if group.trim().is_empty() {
                        None
                    } else {
                        group.chars().skip(1).next()
                    }
                })
                .collect()
        }

        let stack_lines: Vec<Vec<Option<char>>> = input.lines().map(parse_line).collect();

        let mut ship = Ship::new();
        for line in stack_lines {
            for (x, container) in line.iter().enumerate().into_iter() {
                ship.add(x, container);
            }
        }
        ship
    }
}
