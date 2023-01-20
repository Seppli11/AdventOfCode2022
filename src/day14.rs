use std::{
    collections::HashMap,
    fmt::{Display, Write},
    fs,
    ops::{Add, AddAssign, Index, Sub, SubAssign},
};

use itertools::Itertools;

pub fn day14() {
    let input = fs::read_to_string("./input/day14/test-input.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day14/input.txt").expect("Couldn't load input");

    let mut map = Map::new();
    for line in input.lines().map(parse_line) {
        map.draw_line(line);
    }
    map.infinity_y = map.max_y() + 2;
    println!("{map}");

    let sand_spawn_pos = Vector::new(500, 0);
    let mut counter = 1;
    while map.simulate_sand(sand_spawn_pos) != sand_spawn_pos {
        counter += 1;
    }
    println!("{map}");
    println!("counter: {counter}");
}

fn parse_line(line: &str) -> Vec<Vector> {
    line.split("->")
        .filter_map(|pos_str| pos_str.trim().split_once(","))
        .map(|(x, y)| {
            Vector::new(
                x.parse::<i32>().expect("x is not a number"),
                y.parse::<i32>().expect("y is not a number"),
            )
        })
        .collect_vec()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }

    fn normalized(self) -> Vector {
        let length = self.length() as i32;
        Vector::new(self.x.div_floor(length), self.y.div_floor(length))
    }

    fn length(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}/{})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Material {
    Air,
    Sand,
    Stone,
}

impl Material {
    fn get_char(&self) -> char {
        match self {
            Material::Air => '.',
            Material::Sand => '+',
            Material::Stone => '#',
        }
    }
}

struct Map {
    map: HashMap<Vector, Material>,
    infinity_y: i32,
}

impl Map {
    fn new() -> Map {
        Self {
            map: HashMap::new(),
            infinity_y: 0,
        }
    }

    fn draw_line(&mut self, line: Vec<Vector>) {
        let mut current = *line.first().expect("line is empty");
        for next_pos in line.into_iter().skip(1) {
            let direction = (next_pos - current).normalized();
            if direction.x != 0 && direction.y != 0 {
                panic!(
                    "In the direction {:?} both the x and y coordinate are not null",
                    direction
                )
            }
            while current != next_pos {
                self.add(current, Material::Stone);
                current += direction;
            }
            self.add(current, Material::Stone);
        }
    }

    fn add(&mut self, pos: Vector, material: Material) {
        self.map.insert(pos, material);
    }

    fn min_x(&self) -> i32 {
        self.map.keys().map(|pos| pos.x).min().unwrap_or(0)
    }
    fn max_x(&self) -> i32 {
        self.map.keys().map(|pos| pos.x).max().unwrap_or(0)
    }
    fn min_y(&self) -> i32 {
        self.map.keys().map(|pos| pos.y).min().unwrap_or(0)
    }
    fn max_y(&self) -> i32 {
        self.map.keys().map(|pos| pos.y).max().unwrap_or(0)
    }

    fn simulate_sand(&mut self, spawn_pos: Vector) -> Vector {
        let max_y = self.max_y();
        let mut current = spawn_pos;
        let operations = [Vector::new(0, 1), Vector::new(-1, 1), Vector::new(1, 1)];
        loop {
            let op = operations
                .iter()
                .map(|op| current + *op)
                .find(|pos| self[*pos] == Material::Air);

            match op {
                Some(pos) => current = pos,
                None => {
                    self.add(current, Material::Sand);
                    return current;
                }
            }
        }
    }
}

impl Index<Vector> for Map {
    type Output = Material;

    fn index(&self, index: Vector) -> &Self::Output {
        if index.y == self.infinity_y {
            &Material::Stone
        } else {
            self.map.get(&index).unwrap_or(&Material::Air)
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y()..self.max_y() + 3 {
            let line: String = (self.min_x()..self.max_x() + 1)
                .map(|x| Vector::new(x, y))
                .map(|pos| self[pos].get_char())
                .collect();
            writeln!(f, "{line}")?
        }
        Ok(())
    }
}
