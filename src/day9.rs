use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use itertools::Itertools;

pub fn day9() {
    let input = fs::read_to_string("./input/day9/test-input.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day9/input.txt").expect("Couldn't load input");
    let input: Vec<Vector> = input.lines().map(Vector::from_line).collect();

    let mut rope = Rope::new();
    for movement in input {
        rope.apply(movement)
    }
    println!("movement count: {}", rope.tail_pos.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector(i32, i32);

impl Vector {
    fn from_line(line: &str) -> Vector {
        let (move_str, steps) = line.split_once(" ").unwrap();
        let steps: i32 = steps.parse().unwrap();
        match move_str {
            "U" => Vector(0, -steps),
            "D" => Vector(0, steps),
            "L" => Vector(-steps, 0),
            "R" => Vector(steps, 0),
            _ => panic!("unkown movement {move_str}"),
        }
    }
    fn len(&self) -> f64 {
        f64::sqrt((i32::pow(self.0, 2) + i32::pow(self.1, 2)) as f64)
    }

    fn signum(&self) -> Vector {
        Vector(self.0.signum(), self.1.signum())
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Vec({}, {})", self.0, self.1))
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

struct Rope {
    knots: Vec<Vector>,
    tail_pos: HashSet<Vector>,
}

impl Rope {
    fn new() -> Rope {
        let mut tail_pos = HashSet::new();
        tail_pos.insert(Vector(0, 0));
        Rope {
            knots: (0..10).map(|_| Vector(0, 0)).collect(),
            tail_pos,
        }
    }

    fn tail_mut(&mut self) -> &mut Vector {
        self.knots.last_mut().unwrap()
    }

    fn apply(&mut self, movement: Vector) {
        let mut movement = movement;
        while movement != Vector(0, 0) {
            self.knots[0] += movement.signum();
            for i in 1..self.knots.len() {
                let last = &self.knots[i - 1];
                let current = self.knots[i];
                let diff = *last - current;
                if diff.len() >= 2f64 {
                    self.knots[i] += diff.signum();
                }
            }

            self.tail_pos.insert(*self.knots.last().unwrap());

            movement -= movement.signum();
        }
    }
}
