use super::vector3::Vector3;

pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3
}

impl Transform {
    pub fn new(position: Vector3, rotation: Vector3) -> Transform {
        Transform { position, rotation }
    }
}

impl Clone for Transform {
    fn clone(&self) -> Self {
        Self { position: self.position.clone(), rotation: self.rotation.clone() }
    }
}

impl Copy for Transform { }
