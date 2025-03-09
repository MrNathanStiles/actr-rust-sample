
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vector3 {
    pub fn new() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}
/*
impl Clone for Vector3 {
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y, z: self.z }
    }
}
impl Copy for Vector3 { }
*/
