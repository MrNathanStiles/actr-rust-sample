use std::{fmt::Display, ops::{Add, AddAssign, Mul}};


pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
    pub fn zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Clone for Vector3 {
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y, z: self.z }
    }
}

impl Copy for Vector3 { }


impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("x: ").unwrap();
        f.write_str(&self.x.to_string()).unwrap();
        f.write_str(", y: ").unwrap();
        f.write_str(&self.y.to_string()).unwrap();
        f.write_str(", z: ").unwrap();
        f.write_str(&self.z.to_string())
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    
    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}