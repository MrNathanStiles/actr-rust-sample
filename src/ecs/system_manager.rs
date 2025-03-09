use std::{any::TypeId, collections::HashMap};

use super::{component_manager::ComponentManager, coordinator::{self, Coordinator}, signature::Signature, system::System, Entity};

pub struct SystemManager {
    signatures: HashMap<TypeId, Signature>,
    systems: HashMap<usize, System>,
}

impl SystemManager {
    
    pub fn new() -> SystemManager {
        SystemManager {
            signatures: HashMap::new(),
            systems: HashMap::new(),
        }
    }

    pub fn register_system(&mut self, system: System)
    {
        self.systems.insert(system.id, system);
    }
    
    pub fn set_signature<T>(&mut self, signature: Signature)
    where
        T: 'static
    {
        let id = TypeId::of::<T>();
        self.signatures.insert(id, signature);
    }
    
    pub fn entity_destroyed(&mut self, entity: Entity) {
        for (id, system) in self.systems.iter_mut() {
            system.entity_remove(entity);
        }
    }
    
    pub fn entity_signature_changed(&mut self, entity: Entity, entity_signature: Signature) {
        for (id, system) in self.systems.iter_mut() {
            if (system.signature.matches(entity_signature)) {
                system.entity_add(entity);
            } else {
                system.entity_remove(entity);
            }
        }
    }

    pub fn update(&self, component_manager: &mut ComponentManager, delta: f64) {
        for (id, system) in self.systems.iter() {
            (system.update_function)(component_manager, &system.entities, delta)
        }
    }

}