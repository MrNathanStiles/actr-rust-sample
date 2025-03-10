use super::{
    Entity,
    component_manager::{ComponentManager, ComponentType},
    entity_manager::EntityManager,
    signature::Signature,
    system::System,
    system_manager::SystemManager,
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
        self.component_manager.register_component::<T>();
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: T)
    where
        T: 'static,
    {
        self.component_manager.add_component(entity, component);
        let entity_signature = self.entity_manager.get_signature(entity);
        entity_signature.set(self.component_manager.get_component_type::<T>());
        self.system_manager
            .entity_signature_changed(entity, *entity_signature);
    }

    pub fn remove_component<T>(&mut self, entity: Entity)
    where
        T: 'static,
    {
        self.component_manager.remove_component::<T>(entity);
    }

    pub fn get_component<T>(&mut self, entity: Entity) -> &mut T
    where
        T: 'static,
    {
        self.component_manager.get_component::<T>(entity)
    }

    pub fn get_component_type<T>(&self) -> ComponentType
    where
        T: 'static,
    {
        self.component_manager.get_component_type::<T>()
    }
    pub fn register_system(&mut self, system: System) {
        self.system_manager.register_system(system)
    }

    pub fn set_system_signature<T>(&mut self, signature: Signature)
    where
        T: 'static,
    {
        self.system_manager.set_signature::<T>(signature);
    }

    pub fn update(&mut self, delta: f64) {
        self.system_manager
            .update(&mut self.component_manager, delta);
    }
}
