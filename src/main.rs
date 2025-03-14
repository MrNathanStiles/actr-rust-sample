use actr_rust_sample::{di::container::Container, ecs::{component_manager::ComponentManager, coordinator::Coordinator, entity_manager::EntityManager, system_manager::SystemManager}};





fn main() {
    
    Container::initialize();

    let mut container = Container::new();
    container.register_service(EntityManager::new());
    container.register_service(ComponentManager::new());
    container.register_service(SystemManager::new());
    container.register_service(Coordinator::new());

    
}
