
use std::ops::DerefMut;

use crate::di::container::Container;

use super::{
    Entity, component_manager::ComponentManager, entity_manager::EntityManager, system::System,
    system_manager::SystemManager,
};

pub struct Coordinator {}

impl Coordinator {
    pub fn new() -> Coordinator {
        Coordinator {}
    }

    pub fn create_entity(&mut self) -> Entity {
        let container = Container::new();
        let entity_manager = container.get_service::<EntityManager>();
        entity_manager.lock().create_entity()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        let container = Container::new();
        let entity_manager = container.get_service::<EntityManager>();
        let component_manager = container.get_service::<ComponentManager>();
        let system_manager = container.get_service::<SystemManager>();

        
        entity_manager.lock().destroy_entity(entity);
        component_manager.lock().entity_destroyed(entity);
        system_manager.lock().entity_destroyed(entity);
    }

    pub fn register_component<T>(&mut self)
    where
        T: 'static + Send + Clone,
    {
        let container = Container::new();
        let component_manager = container.get_service::<ComponentManager>();
        component_manager.lock().register_component::<T>();
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: T)
    where
        T: 'static + Send,
    {
        let container = Container::new();
        let entity_manager = container.get_service::<EntityManager>();
        let mut em = entity_manager.lock();
        let component_manager = container.get_service::<ComponentManager>();
        let system_manager = container.get_service::<SystemManager>();

        component_manager.lock().add_component(entity, component);
        let entity_signature = em.get_signature(entity);
        entity_signature.set(component_manager.lock().get_component_type::<T>());
        system_manager.lock().entity_signature_changed(entity, *entity_signature);
    }

    pub fn remove_component<T>(&mut self, entity: Entity)
    where
        T: 'static + Send + Clone,
    {
        let container = Container::new();
        let component_manager = container.get_service::<ComponentManager>();

        component_manager.lock().remove_component::<T>(entity);
    }

    pub fn register_system(&mut self, system: System) {
        let container = Container::new();
        let system_manager = container.get_service::<SystemManager>();
        let mut system_manager = system_manager.lock();
        system_manager.register_system(system)
    }

    pub fn update(&mut self, delta: f64) {
        let container = Container::new();
        let system_manager = container.get_service::<SystemManager>();
        let system_manager = system_manager.lock();

        for (_id, system) in system_manager.systems.iter() {
            (system.update_function)(&system.entities, delta);
        }
    }
}
