use super::{component_manager::ComponentManager, entity_manager::EntityManager, system_manager::SystemManager};

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
}
