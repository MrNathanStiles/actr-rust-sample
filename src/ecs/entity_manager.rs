use queues::{IsQueue, Queue};
use crate::actr::_actr_log_length;

use super::{signature::Signature, Entity, MAX_ENTITIES};

pub struct EntityManager {
    available: Queue<Entity>,
    alive: Entity,
    signatures: Vec<Signature>
}

impl EntityManager {

    pub fn new() -> EntityManager {
        let mut result = EntityManager {
            available: Queue::new(),
            alive: 0,
            signatures: Vec::with_capacity(MAX_ENTITIES)
        };
        for n in 0..MAX_ENTITIES {
            result.available.add(n).unwrap();
            result.signatures.push(Signature::new(0));
        }
        result
    }
    pub fn log(&self, message: String) {
        unsafe {
            _actr_log_length(message.as_ptr(), message.len());
        }
    }
    pub fn create_entity(&mut self) -> Entity {
        let id = self.available.remove().unwrap();
        self.alive += 1;
        // self.log(format!("entity created {id}"));
        id
    }
    
    pub fn destroy_entity(&mut self, entity: Entity) {
        self.signatures[entity] = Signature::zero();
        self.available.add(entity).unwrap();
        self.alive -= 1;
    }

    pub fn get_signature(&mut self, entity: Entity) -> &mut Signature {
        self.signatures.get_mut(entity).unwrap()
    }
    
}
