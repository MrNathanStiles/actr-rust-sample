use std::collections::HashSet;

use super::{coordinator::Coordinator, signature::Signature, Entity};

static mut OBJECT_COUNTER: usize = 0;

pub struct System {
    pub id: usize,
    pub signature: Signature,
    update_function: fn(&mut Coordinator, &HashSet<Entity>, f64) -> (),
    entities: HashSet<Entity>,
}

impl System {
    pub fn new(signature: Signature, update_function: fn(&mut Coordinator, &HashSet<Entity>, f64) -> ()) -> System {
        unsafe { 
            let result = System {
                id: OBJECT_COUNTER,
                signature,
                update_function,
                entities: HashSet::new(),
            };
            OBJECT_COUNTER += 1; 
            result
        }
    }
    pub fn entity_remove(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }
    pub fn entity_add(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }
}
