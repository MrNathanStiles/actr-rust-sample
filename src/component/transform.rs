use super::vector3::Vector3;

pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3
}

impl Transform {
    pub fn new(x: f64, y: f64, z: f64) -> Transform {
        Transform { position: Vector3::new(x, y, z), rotation: Vector3::zero() }
    }
}

impl Clone for Transform {
    fn clone(&self) -> Self {
        Self { position: self.position.clone(), rotation: self.rotation.clone() }
    }
}

impl Copy for Transform { }
