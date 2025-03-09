use super::{
    component_array::ComponentArray, component_manager::ComponentManager, entity_manager::EntityManager, system::System, system_manager::SystemManager, Entity, Signature
};

pub struct Coordinator {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: SystemManager,
}

impl Coordinator {
    pub fn new() -> Coordinator {
        Coordinator {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: SystemManager::new(),
        }
    }
    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }
    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entity_manager.destroy_entity(entity);
        self.component_manager.entity_destroyed(entity);
        self.system_manager.entity_destroyed(entity);
    }
    pub fn register_component<T>(&mut self)
    where
        T: 'static,
    {
        let ca = ComponentArray::<T>::new();
        self.component_manager.register_component(ca);
    }
    pub fn add_component<T>(&mut self, entity: Entity, component: &mut T)
    where
        T: 'static,
    {
        self.component_manager.add_component(entity, component);
    }
    pub fn remove_component<T>(&mut self, entity: Entity)
    where
        T: 'static
    {
        self.component_manager.remove_component::<T>(entity);
    }
    pub fn get_component<T>(&mut self, entity: Entity) -> &mut T
    where
        T: 'static
    {
        self.component_manager.get_component::<T>(entity)
    }
    pub fn get_component_type<T>(&self)
    where
        T: 'static
    {
        self.component_manager.get_component_type::<T>();
    }
    pub fn register_system<T>(&mut self, system: System)
    where
        T: 'static
    {
            self.system_manager.register_system::<T>(system)
    }

    pub fn set_system_signature<T>(&mut self, signature: Signature)
    where 
        T: 'static
    {
        self.system_manager.set_signature::<T>(signature);
    }
}
