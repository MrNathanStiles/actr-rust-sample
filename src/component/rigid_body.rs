use super::vector3::Vector3;

pub struct RigidBody {
    pub velocity: Vector3,
    pub angularVelocity: Vector3,
}

impl RigidBody {
    pub fn new(velocity: Vector3, angularVelocity: Vector3) -> RigidBody {
        RigidBody {
            velocity,
            angularVelocity,
        }
    }
}
