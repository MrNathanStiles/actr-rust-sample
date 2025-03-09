use crate::ecs::{component_manager::ComponentManager, entity_manager::EntityManager, system_manager::SystemManager};

pub struct State {
    pub component_manager: ComponentManager,
    pub entity_manager: EntityManager,
    pub system_manager: SystemManager,
}

impl State {
    pub fn new() -> State {
        State {
            component_manager: ComponentManager::new(),
            entity_manager: EntityManager::new(),
            system_manager: SystemManager::new(),
        }
    }
}
