use std::{fmt::Display, fs};

use itertools::FoldWhile;
use itertools::Itertools;

pub fn day8() {
    let input = fs::read_to_string("./input/day8/test-input.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day8/input.txt").expect("Couldn't load input");

    let map = Map::from_string(&input);
    println!("trees:  {:?}", map);
    println!("first row: {:?}", map.row(0));
    println!("first col: {:?}", map.col(0));

    let mut count = map.width() * 2 + (map.height() - 2) * 2;
    for y in 1..map.height() - 1 {
        for x in 1..map.height() - 1 {
            if map.is_visible((x, y)) {
                count += 1;
            }
        }
    }

    println!("count: {count}");
    //println!("count: {map}");

    let tree_map: Vec<Vec<u32>> = map
        .trees
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(col_index, _)| {
                    if map.is_visible((col_index, row_index)) {
                        1
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();

    let lines = tree_map.iter().map(|row| row.iter().join("")).join("\n");

    map.get_senic_score((1, 2));

    let max_scenic_score: u32 = map
        .trees
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(col_index, _)| map.get_senic_score((col_index, row_index)))
                .collect::<Vec<u32>>()
        })
        .max()
        .unwrap();
    println!("scenic score: {max_scenic_score}")
}

#[derive(Debug)]
struct Map {
    trees: Vec<Vec<u32>>,
}

type Pos = (usize, usize);

impl Map {
    fn from_string(input: &str) -> Map {
        let trees = input
            .lines()
            .map(|row| row.chars().map(|char| char.to_digit(10).unwrap()).collect())
            .collect();
        Map { trees }
    }

    fn get(&self, (x, y): Pos) -> u32 {
        self.trees[y as usize][x as usize]
    }

    fn row(&self, row: usize) -> Vec<u32> {
        self.trees[row].iter().map(|val| *val).collect()
    }

    fn col(&self, col: usize) -> Vec<u32> {
        self.trees.iter().map(|col_vec| col_vec[col]).collect()
    }

    fn width(&self) -> usize {
        self.trees[0].len()
    }

    fn height(&self) -> usize {
        self.trees.len()
    }

    fn is_visible(&self, (x, y): Pos) -> bool {
        let max_height = self.get((x, y));
        let left = self
            .row(y)
            .iter()
            .take(x as usize)
            .all(|height| height < &max_height);
        let right = self
            .row(y)
            .iter()
            .skip(x + 1)
            .all(|height| height < &max_height);

        let up = self
            .col(x)
            .iter()
            .take(y as usize)
            .all(|height| height < &max_height);
        let down = self
            .col(x)
            .iter()
            .skip(y + 1)
            .all(|height| height < &max_height);
        left | right | up | down
    }

    fn get_senic_score(&self, (x, y): Pos) -> u32 {
        let max_height = self.get((x, y));
        let left = self
            .row(y)
            .iter()
            .take(x)
            .rev()
            .fold_while(0, |count, current| {
                if *current < max_height {
                    FoldWhile::Continue(count + 1)
                } else {
                    FoldWhile::Done(count + 1)
                }
            })
            .into_inner();
        let right = self
            .row(y)
            .iter()
            .skip(x + 1)
            .fold_while(0, |count, current| {
                if *current < max_height {
                    FoldWhile::Continue(count + 1)
                } else {
                    FoldWhile::Done(count + 1)
                }
            })
            .into_inner();
        let up = self
            .col(x)
            .iter()
            .take(y)
            .rev()
            .fold_while(0, |count, current| {
                if *current < max_height {
                    FoldWhile::Continue(count + 1)
                } else {
                    FoldWhile::Done(count + 1)
                }
            })
            .into_inner();
        let down = self
            .col(x)
            .iter()
            .skip(y + 1)
            .fold_while(0, |count, current| {
                if *current < max_height {
                    FoldWhile::Continue(count + 1)
                } else {
                    FoldWhile::Done(count + 1)
                }
            })
            .into_inner();
        //println!("{x}/{y}: {max_height}:  left: {left}, right: {right}, up: {up}, down: {down}, score: {}", left * right * up * down);
        left * right * up * down
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self.trees.iter().map(|row| row.iter().join("")).join("\n");
        f.write_str(&lines)?;
        Ok(())
    }
}
