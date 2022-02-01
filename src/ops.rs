use std::{fmt::Display, cmp::Ordering, ops::{Add, Sub, AddAssign, SubAssign, Mul, Neg, Index}};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}


impl Point {   
    pub fn new(x: i32, y:i32) -> Self {
        Self {
            x, 
            y
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 1,
            y: 1
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point -> X = {}, Y = {}", self.x, self.y)       
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        
        if self.x > other.x {
            Some(Ordering::Greater)
        } else if self.x == other.x {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;        
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: self.x * -1,
            y: self.y * -1
        }
    }
}

impl Index<u8> for Point {
    type Output = i32;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            _ => &self.y
        }
    }
}