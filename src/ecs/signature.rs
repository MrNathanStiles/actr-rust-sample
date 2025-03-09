use super::component_manager::ComponentType;

pub struct Signature {
    value: u64,
}
impl Clone for Signature {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}
impl Copy for Signature {
    
}

impl Signature {
    
    pub fn zero() -> Signature {
        Signature::new(0)
    }
    
    pub fn new(value: u64) -> Signature {
        Signature {
            value
        }
    }

    pub fn set(&mut self, bit: ComponentType) {
        let mask: u64 = 1 << bit;
        self.value |= mask;
    }

    pub fn unset(&mut self, bit: ComponentType) {
        let mask: u64 = 1<< bit;
        self.value &= mask;
    }

    pub fn matches(&self, other: Signature) -> bool {
        (other.value & self.value) == self.value
    }

}