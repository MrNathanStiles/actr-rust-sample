mod actr;
mod component;
mod di;
mod ecs;

use actr::{_actr_log_length, actr_performance};
use component::{gravity::Gravity, identity::Identity, rigid_body::RigidBody, transform::Transform};
use di::container::Container;
use ecs::{
    Entity, component_manager::ComponentManager, coordinator::Coordinator,
    entity_manager::EntityManager, signature::Signature, system::System,
    system_manager::SystemManager,
};
use std::collections::HashSet;

fn update_sample_system(entities: &HashSet<Entity>, delta: f64) {
    let container = Container::new();
    let component_manager = container.get_service::<ComponentManager>();
    let mut remove = HashSet::new();
    
    for entity in entities {
        let transform = component_manager.get_component::<Transform>(*entity);
        let gravity = component_manager.get_component::<Gravity>(*entity);
        let rigid_body = component_manager.get_component::<RigidBody>(*entity);
        let identity = component_manager.get_component::<Identity>(*entity);

        if identity.entity != *entity {
            panic!("FAIL FAIL");
        } else {
            log(format!("facts {entity}"));
        }

        rigid_body.velocity += gravity.direction * delta;
        transform.position += rigid_body.velocity * delta;

        if transform.position.y < 0.0 {
            remove.insert(entity);
        }
    }

    if remove.len() > 0 {
        let coordinator = container.get_service::<Coordinator>();
        for entity in remove {
            coordinator.destroy_entity(*entity);
            add_entity();
        }
    }
}

fn register_sample_system() {
    let container = Container::new();
    let component_manager = container.get_service::<ComponentManager>();
    let coordinator = container.get_service::<Coordinator>();

    let mut signature = Signature::zero();
    signature.set(component_manager.get_component_type::<Transform>());
    signature.set(component_manager.get_component_type::<Gravity>());
    signature.set(component_manager.get_component_type::<RigidBody>());
    signature.set(component_manager.get_component_type::<Identity>());
    let sample_system = System::new(signature, update_sample_system);
    coordinator.register_system(sample_system);
}

fn add_entity() {
    let container = Container::new();
    let coordinator = container.get_service::<Coordinator>();

    let entity = coordinator.create_entity();
    coordinator.add_component(entity, Transform::new(0.0, 100.0, 0.0));
    coordinator.add_component(entity, Gravity::new());
    coordinator.add_component(entity, RigidBody::new());
    coordinator.add_component(entity, Identity { entity });

}
#[unsafe(no_mangle)]
pub extern fn actr_init(_w: f32, _h: f32) {
    Container::initialize();
    let mut container = Container::new();
    container.register_service(EntityManager::new());
    container.register_service(ComponentManager::new());
    container.register_service(SystemManager::new());
    container.register_service(Coordinator::new());

    let coordinator = container.get_service::<Coordinator>();
    coordinator.register_component::<Transform>();
    coordinator.register_component::<Gravity>();
    coordinator.register_component::<RigidBody>();
    coordinator.register_component::<Identity>();

    register_sample_system();

    for _n in 0..1024 {
        add_entity();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn actr_step(delta: f64) {
    let container = Container::new();
    let coordinator = container.get_service::<Coordinator>();

    let start = unsafe { actr_performance() };

    coordinator.update(delta);

    let end = unsafe { actr_performance() };

    let time = end - start;

    log(format!("time {time}"));
}

pub fn log(message: String) {
    unsafe {
        _actr_log_length(message.as_ptr(), message.len());
    }
}
