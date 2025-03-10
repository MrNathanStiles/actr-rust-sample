use super::vector3::Vector3;

pub struct RigidBody {
    pub velocity: Vector3
}

impl RigidBody {
    pub fn new() -> RigidBody {
        RigidBody {
            velocity: Vector3::zero()
        }
    }
}
