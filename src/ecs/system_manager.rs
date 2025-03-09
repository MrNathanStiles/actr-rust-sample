use std::{any::TypeId, collections::HashMap};

use super::{system::System, Entity, Signature};

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
    pub fn register_system<T>(&mut self, system: System)
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
            let system_signature = system.signature;
            if ((entity_signature & system_signature) == system_signature) {
                system.entity_add(entity);
            } else {
                system.entity_remove(entity);
            }
        }
    }

}