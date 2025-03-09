use crate::ecs::{component_manager::ComponentManager, coordinator::Coordinator, entity_manager::EntityManager, system_manager::SystemManager};

pub struct State {
    pub ecs: Coordinator,
}

impl State {
    pub fn new() -> State {
        State {
            ecs: Coordinator::new()
        }
    }
}
