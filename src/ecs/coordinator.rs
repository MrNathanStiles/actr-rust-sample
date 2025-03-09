use super::{component_manager::ComponentManager, entity_manager::EntityManager, system_manager::SystemManager, Entity};

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
        self.component_manager.e
    }
}
