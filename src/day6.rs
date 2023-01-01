use std::{collections::HashSet, fs};

use itertools::Itertools;
use tuple_map::TupleMap4;

pub fn day6() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let input = fs::read_to_string("./input/day6/input.txt").expect("Couldn't load input");
    let chars: Vec<char> = input.chars().collect();
    for i in 14..chars.len() {
        let len = chars[i - 14..i].iter().sorted().dedup().count();
        if (len == 14) {
            println!("pos: {}", i);
            break;
        }
    }
}
