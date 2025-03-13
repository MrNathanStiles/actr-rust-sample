use super::{
    component_manager::ComponentManager, coordinator::Coordinator, entity_manager::EntityManager,
    system_manager::SystemManager, Entity,
};  

pub enum Message {
    Coordinator(Box<dyn Fn(&mut Coordinator)>),
    EntityManager(Box<dyn Fn(&mut EntityManager)>),
    ComponentManager(Box<dyn Fn( &mut ComponentManager)>),
    SystemManager(Box<dyn Fn(&mut SystemManager)>),
    CoorECS(Box<dyn Fn(&mut Coordinator, &mut EntityManager, &mut ComponentManager, &mut SystemManager)>),
    DestroyEntity(Entity)
}
