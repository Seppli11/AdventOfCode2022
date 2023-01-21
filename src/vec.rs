use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }

    pub fn normalized(self) -> Vector {
        let length = self.length() as i32;
        Vector::new(self.x.div_floor(length), self.y.div_floor(length))
    }

    pub fn abs(self) -> Vector {
        Vector::new(self.x.abs(), self.y.abs())
    }

    pub fn length(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    pub fn manhattan_distance(&self, other: &Vector) -> u32 {
        let diff = (*self - *other).abs();
        (diff.x + diff.y) as u32
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
