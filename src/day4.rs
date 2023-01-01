use itertools::Itertools;
use std::{collections::HashSet, fs, iter::Map};
use tuple_map::{self, TupleMap2};

type Range = (u32, u32);

trait RangeEx {
    fn len(&self) -> usize;

    fn contains(&self, num: u32) -> bool;
}
impl RangeEx for Range {
    fn len(&self) -> usize {
        self.0.abs_diff(self.1) as usize
    }

    fn contains(&self, num: u32) -> bool {
        self.0 <= num && self.1 >= num
    }
}

pub fn day4() {
    let input = fs::read_to_string("./input/day4/test.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day4/input.txt").expect("Couldn't load input");
    let count = input.lines().map(parse_line).filter(is_overlapping).count();
    println!("count: {count}")
}

fn parse_line(line: &str) -> (Range, Range) {
    line.split_once(",").unwrap().map(parse_range)
}

fn parse_range(range: &str) -> Range {
    range.split_once("-").unwrap().map(|v| v.parse().unwrap())
}

fn is_contained((fst, snd): &(Range, Range)) -> bool {
    let (smaller, bigger) = if fst.len() < snd.len() {
        (fst, snd)
    } else {
        (snd, fst)
    };

    smaller.0 >= bigger.0 && smaller.1 <= bigger.1
}

fn is_overlapping((fst, snd): &(Range, Range)) -> bool {
    fst.contains(snd.0) || fst.contains(snd.1) || snd.contains(fst.0) || snd.contains(fst.1)
}
