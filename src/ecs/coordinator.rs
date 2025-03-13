use std::sync::mpsc::Sender;

use super::{Entity, message::Message};

pub struct Coordinator {
    sender: Sender<Message>,
}

impl Coordinator {
    pub fn new(sender: Sender<Message>) -> Coordinator {
        Coordinator { sender }
    }

    pub fn add_component<T>(&self, entity: Entity, component: T)
    where
        T: 'static + Send + Copy,
    {
        let result = self
            .sender
            .send(Message::CoorECS(Box::new(move |coor, em, cm, sm| {
                cm.add_component(entity, component);
                let entity_signature = em.get_signature(entity);
                entity_signature.set(cm.get_component_type::<T>());
                sm.entity_signature_changed(entity, *entity_signature);
            })));
        result.unwrap();
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.sender.send(Message::DestroyEntity(entity)).unwrap();
    }
    pub fn update(&mut self, delta: f64) {
        let result = self
            .sender
            .send(Message::CoorECS(Box::new(move |coor, em, cm, sm| {
                for (_id, system) in sm.systems.iter() {
                    (system.update_function)(cm, &system.entities, delta);
                }
            })));

        result.unwrap();
    }
}
