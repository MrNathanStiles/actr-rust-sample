mod actr;
mod component;
mod di;
mod ecs;

use actr::{_actr_log_length, actr_performance};
use component::{gravity::Gravity, rigid_body::RigidBody, transform::Transform};
use di::container::Container;
use ecs::{
    component_manager::{self, ComponentManager}, coordinator::{self, Coordinator}, entity_manager::EntityManager, signature::Signature, system::System, system_manager::SystemManager, Entity
};
use std::collections::HashSet;

fn update_sample_system(entities: &HashSet<Entity>, delta: f64) {
    let container = Container::new();
    let component_manager = container.get_service::<ComponentManager>();
    let mut cm = component_manager.lock();
    let mut remove = HashSet::new();
    for entity in entities {
        let transform = cm.get_component::<Transform>(*entity);
        let gravity = cm.get_component::<Gravity>(*entity);
        let rigid_body = cm.get_component::<RigidBody>(*entity);

        if (transform.position.y < 0.0) {
            panic!("not fine");
        }
        rigid_body.velocity += gravity.direction * delta;
        transform.position += rigid_body.velocity * delta;

        if transform.position.y < 0.0 {
            remove.insert(entity);
        }
    }

    if remove.len() > 0 {
        let coordinator = container.get_service::<Coordinator>();
        let mut coordinator = coordinator.lock();
        for entity in remove {
            coordinator.destroy_entity(*entity);
            add_entity();
        }
    }
}

fn register_sample_system() {
    /*
    let container = Container::new();
    let component_manager = container.get_service::<ComponentManager>();
    let coordinator = container.get_service::<Coordinator>();


    let mut signature = Signature::zero();
    signature.set(component_manager.get_component_type::<Transform>());
    signature.set(component_manager.get_component_type::<Gravity>());
    signature.set(component_manager.get_component_type::<RigidBody>());
    let sample_system = System::new(signature, update_sample_system);
    coordinator.register_system(sample_system);
    */
}

fn add_entity() {
    /*
    let container = Container::new();
    let coordinator = container.get_service::<Coordinator>();
    let component_manager = container.get_service::<ComponentManager>();
    let entity = coordinator.create_entity();
    let y = 100.0;
    coordinator.add_component(entity, Transform::new(0.0, y, 0.0));
    coordinator.add_component(entity, Gravity::new());
    coordinator.add_component(entity, RigidBody::new());

    let transform = component_manager.get_component::<Transform>(entity);
    if transform.position.y != y {
        panic!("not fine");
    }*/

}




#[unsafe(no_mangle)]
pub extern fn actr_init(_w: f32, _h: f32) {
    
    let mut container = Container::new();
    container.register_service(EntityManager::new());
    container.register_service(ComponentManager::new());
    container.register_service(SystemManager::new());
    container.register_service(Coordinator::new());

    let coordinator = container.get_service::<Coordinator>();
    let mut coordinator = coordinator.lock();
    
    coordinator.register_component::<Transform>();
    coordinator.register_component::<Gravity>();
    coordinator.register_component::<RigidBody>();

    register_sample_system();

    for _n in 0..1024 {
        add_entity();
    }

    let value = vec![1, 2, 3];
    
    container.register_service(value);

    let vec = container.get_service::<Vec<i32>>();
    let vec2 = vec.lock();
    let slice = vec2.as_slice();
    let vec = container.get_service::<Vec<i32>>();
    let mut vec2 = vec.lock();
    vec2.extend([4, 5, 6, 7, 8, 9, 10]);
    
    log(format!("v1 {slice:?}"));


}

#[unsafe(no_mangle)]
pub extern "C" fn actr_step(delta: f64) {
    let container = Container::new();
    let coordinator = container.get_service::<Coordinator>();

    let start = unsafe { actr_performance() };

    // coordinator.update(delta);

    let end = unsafe { actr_performance() };

    let time = end - start;

    log(format!("time {time}"));

    let value = vec![1, 2, 3];
}

pub fn log(message: String) {
    unsafe {
        _actr_log_length(message.as_ptr(), message.len());
    }
}
