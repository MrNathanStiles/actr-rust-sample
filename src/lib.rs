mod actr;
mod component;
mod di;
mod ecs;

use std::{collections::HashSet, hash::Hash};

use actr::{_actr_log_length, actr_performance};
use component::{gravity::Gravity, transform::Transform};
use di::container::Container;
use ecs::{
    component_manager::ComponentManager, coordinator::{self, Coordinator}, entity_manager::EntityManager, signature::Signature, system::System, system_manager::SystemManager, Entity
};

fn update_sample_system(entities: &HashSet<Entity>, delta: f64) {
    let container = Container::new();
    let component_manager = container.get_service::<ComponentManager>();
    let mut remove = HashSet::new();
    for entity in entities {
        //log(format!("entity {entity}"));
        let transform = component_manager.get_component::<Transform>(*entity);
        if transform.position.y < 0.0 {
            let y = transform.position.y;
            log(format!("entity {entity} FAIL FAIL {y}"));
            panic!("entity {entity} FAIL FAIL");
        }
        let gravity = component_manager.get_component::<Gravity>(*entity);
        //let direction = gravity.direction;
        //log(format!("gravity {direction}"));
        transform.position += gravity.direction * delta;
        
        //log(format!("entity {entity} position {position}"));
        if transform.position.y < 0.0 {
            log(format!("entity {entity} to be removed"));
            remove.insert(entity);
        }
    }
    if remove.len() > 0 {
        let coordinator = container.get_service::<Coordinator>();
        for entity in remove {
            coordinator.destroy_entity(*entity);
            log(format!("entity destroyed {entity}"));
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
    let sample_system = System::new(signature, update_sample_system);
    coordinator.register_system(sample_system);
}

fn add_entity() {
    let container = Container::new();
    let coordinator = container.get_service::<Coordinator>();
    let component_manager = container.get_service::<ComponentManager>();
    
    let entity = coordinator.create_entity();
    coordinator.add_component(entity, Transform::new(0.0, 0.1, 0.0));
    coordinator.add_component(entity, Gravity::new());

    let transform = component_manager.get_component::<Transform>(entity);
    let position = transform.position;

    if position.y != 0.1 {
        panic!("DATA MISMATCH");
    }
    //log(format!("entity added {entity}"));
    //log(format!("position {position}"));
}
#[unsafe(no_mangle)]
pub extern "C" fn actr_init(_w: f32, _h: f32) {
    Container::initialize();
    let mut container = Container::new();
    container.register_service(EntityManager::new());
    container.register_service(ComponentManager::new());
    container.register_service(SystemManager::new());
    container.register_service(Coordinator::new());

    let coordinator = container.get_service::<Coordinator>();
    coordinator.register_component::<Transform>();
    coordinator.register_component::<Gravity>();

    register_sample_system();

    for _n in 0..128 {
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
