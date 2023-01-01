use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input/day1/input.txt").expect("Couldn't load input");
    let arrs: Vec<Vec<String>> = input.lines().fold(vec![vec![]], |mut arrs, line| {
        if line.trim().is_empty() {
            arrs.push(Vec::new())
        } else {
            arrs.last_mut().expect("Empty array").push(line.to_string())
        }
        arrs
    });
    let mut sums: Vec<i32> = arrs
        .iter()
        .map(|line_chunk| {
            line_chunk
                .iter()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect();
    sums.sort();
    let max = sums.iter().max().unwrap();
    let index = sums.iter().position(|el| el == max).unwrap();
    let top_three: i32 = sums.iter().rev().take(3).sum();
    println!("max is {max}, index: {index}, topThree: {top_three}")
}
