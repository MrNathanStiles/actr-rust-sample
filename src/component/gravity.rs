
use crate::actr::actr_prng;

use super::vector3::Vector3;

pub struct Gravity {
    pub direction: Vector3,
}

impl Gravity {
    fn random() -> f64 {
        unsafe { actr_prng() }
    }

    pub fn zero() -> Gravity {
        let direction = Vector3::new(0.0, 0.0, 0.0);
        let result = Gravity { direction };
        //log(format!("g {direction}"));
        result
    }
    pub fn new() -> Gravity {
        let y: f64 = Gravity::random() * -10.0;
        let direction = Vector3::new(0.0, y, 0.0);
        let result = Gravity { direction };
        //log(format!("g {direction}"));
        result
    }
}

impl Clone for Gravity {
    fn clone(&self) -> Self {
        Self { direction: self.direction.clone() }
    }
}
impl Copy for Gravity {
    
}