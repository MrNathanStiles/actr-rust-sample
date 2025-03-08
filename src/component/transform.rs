use crate::data::vector3::Vector3;

pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3
}
impl Transform {
    pub fn new() -> Transform {
        Transform { position: Vector3::new(), rotation: Vector3::new() }
    }
}

impl Clone for Transform {
    fn clone(&self) -> Self {
        Self { position: self.position.clone(), rotation: self.rotation.clone() }
    }
}
impl Copy for Transform { }

