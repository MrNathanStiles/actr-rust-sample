use std::sync::mpsc::{channel, Receiver, Sender};

use super::{component_manager::ComponentManager, coordinator::{self, Coordinator}, entity_manager::EntityManager, message::{self, Message}, system_manager::SystemManager, Entity};

pub struct Processor {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    coordinator: Coordinator,
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: SystemManager

}

unsafe impl Send for Processor {
    
}

impl Processor {

    pub fn new() -> Processor {
        let (sender, receiver) = channel::<Message>();
        
        let coordinator = Coordinator::new(sender.clone());
        let entity_manager = EntityManager::new(sender.clone());
        let component_manager = ComponentManager::new(sender.clone());
        let system_manager = SystemManager::new(sender.clone());
        
        Processor {
            sender,
            receiver,
            coordinator,
            entity_manager,
            component_manager,
            system_manager,
        }
    }

    pub fn send_message(&self, message: Message) -> bool {
        let result = self.sender.send(message);
        if result.is_err() {
            false
        } else {
            true
        }
    }

    pub fn process_messages(&mut self) {
        loop {
            let result = self.receiver.try_recv();
            if result.is_err() { break; }
            let message = result.unwrap();
            match message {
                Message::Coordinator(callback) => callback(&mut self.coordinator),
                Message::EntityManager(callback) => callback(&mut self.entity_manager),
                Message::ComponentManager(callback) => callback(&mut self.component_manager),
                Message::SystemManager(callback) => callback(&mut self.system_manager),
                Message::CoorECS(callback) => callback(&mut self.coordinator, &mut self.entity_manager, &mut self.component_manager, &mut self.system_manager),
                Message::DestroyEntity(entity) => {
                    self.entity_manager.destroy_entity(entity);
                    self.component_manager.entity_destroyed(entity);
                    self.system_manager.entity_destroyed(entity);
                }
            }
        }
    }
}
