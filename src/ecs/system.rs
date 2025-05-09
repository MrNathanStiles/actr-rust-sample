use std::collections::HashSet;

use super::{component_manager::ComponentManager, coordinator::Coordinator, entity_manager::EntityManager, signature::Signature, system_manager::SystemManager, Entity};

static mut OBJECT_COUNTER: usize = 0;



pub struct System {
    pub id: usize,
    pub signature: Signature,
    pub update_function: fn(&mut Coordinator, &mut EntityManager, &mut ComponentManager, &mut SystemManager, &HashSet<Entity>, f64) -> (),
    pub entities: HashSet<Entity>,
}

impl System {
    pub fn new(signature: Signature, update_function: fn(&mut Coordinator, &mut EntityManager, &mut ComponentManager, &mut SystemManager, &HashSet<Entity>, f64) -> ()) -> System {
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
