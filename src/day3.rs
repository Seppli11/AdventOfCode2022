use itertools::Itertools;
use std::{collections::HashSet, fs, iter::Map};
use tuple_map::{self, TupleMap2};

pub fn day3() {
    let input = fs::read_to_string("./input/day3/day3.txt").expect("Couldn't load input");
    let sum: u32 = input
        .lines()
        .map(split_in_halve)
        .map(|tuple| tuple.map(sort_string))
        .flat_map(calculate_duplicated)
        .map(get_priority)
        .sum();

    println!("sum: {sum}");
}

pub fn day3b() {
    let input = fs::read_to_string("./input/day3/day3.txt").expect("Couldn't load input");
    let sum: u32 = input
        .lines()
        .map(sort_string)
        .window(3)
        .flat_map(calculate_duplicated_in_array)
        .map(get_priority)
        .sum();

    println!("sum: {sum}")
}

fn split_in_halve(str: &str) -> (&str, &str) {
    let mid_point = str.len() / 2;
    (&str[0..mid_point], &str[mid_point..])
}

fn sort_string(str: &str) -> Vec<char> {
    let mut chars: Vec<char> = str.chars().collect();
    chars.sort();
    chars
}

fn calculate_duplicated((fst, snd): (Vec<char>, Vec<char>)) -> Vec<char> {
    let mut fst_i = 0;
    let mut snd_i = 0;

    let mut duplicates = HashSet::new();
    while fst_i < fst.len() && snd_i < snd.len() {
        let fst_char = fst[fst_i];
        let snd_char = snd[snd_i];
        if fst_char == snd_char {
            duplicates.insert(fst_char);
            fst_i += 1;
            snd_i += 1;
        } else if fst_char > snd_char {
            snd_i += 1;
        } else if fst_char < snd_char {
            fst_i += 1;
        }
    }

    duplicates.into_iter().collect()
}

fn calculate_duplicated_in_array(arr: Vec<Vec<char>>) -> Vec<char> {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut i3 = 0;

    let arr1 = &arr[0];
    let arr2 = &arr[1];
    let arr3 = &arr[2];

    let mut duplicates = HashSet::new();
    while i1 < arr1.len() && i2 < arr2.len() && i3 < arr3.len() {
        let char1 = arr1[i1];
        let char2 = arr2[i2];
        let char3 = arr3[i3];
        if char1 == char2 && char2 == char3 {
            duplicates.insert(char1);
            i1 += 1;
            i2 += 1;
            i3 += 1;
        }
        if char1 < char2 || char1 < char3 {
            i1 += 1;
        }
        if char2 < char1 || char2 < char3 {
            i2 += 1;
        }
        if char3 < char2 || char3 < char1 {
            i3 += 1;
        }
    }

    duplicates.into_iter().collect()
}

fn get_priority(ch: char) -> u32 {
    let ascii = ch as u32;
    if ch.is_lowercase() {
        ascii - 97 + 1
    } else {
        ascii - 65 + 27
    }
}

struct Window<I> {
    iter: I,
    window_size: u32,
}

impl<I> Iterator for Window<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        let mut vec = Vec::with_capacity(self.window_size as usize);
        for _ in 0..self.window_size {
            match self.iter.next() {
                Some(el) => vec.push(el),
                None => return None,
            }
        }
        Some(vec)
    }
}

trait Windowable<I> {
    fn window(self, window_size: u32) -> Window<I>;
}

impl<I> Windowable<I> for I
where
    I: Iterator,
{
    fn window(self, window_size: u32) -> Window<I> {
        Window {
            iter: self,
            window_size,
        }
    }
}
