use std::{collections::HashMap, sync::mpsc::Sender};

use super::{message::Message, signature::Signature, system::System, Entity};

pub struct SystemManager {
    sender: Sender<Message>,
    pub systems: HashMap<usize, System>,
}

impl SystemManager {
    
    pub fn new(sender: Sender<Message>) -> SystemManager {
        SystemManager {
            sender,
            systems: HashMap::new(),
        }
    }

    pub fn register_system(&mut self, system: System)
    {
        self.systems.insert(system.id, system);
    }
    
    pub fn entity_destroyed(&mut self, entity: Entity) {
        for (_id, system) in self.systems.iter_mut() {
            system.entity_remove(entity);
        }
    }
    
    pub fn entity_signature_changed(&mut self, entity: Entity, entity_signature: Signature) {
        for (_id, system) in self.systems.iter_mut() {
            if system.signature.matches(entity_signature) {
                system.entity_add(entity);
            } else {
                system.entity_remove(entity);
            }
        }
    }
    
    

}