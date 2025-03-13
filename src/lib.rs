mod actr;
mod component;
mod di;
mod ecs;

use core::panic::PanicInfo;
use std::{collections::HashSet, panic, sync::LazyLock};

use actr::{_actr_log_length, actr_performance};
use component::{gravity::Gravity, rigid_body::RigidBody, transform::Transform};
use di::container::Container;
use ecs::{
    Entity,
    component_manager::ComponentManager,
    coordinator::Coordinator,
    entity_manager::{self, EntityManager},
    processor::Processor,
    signature::Signature,
    system::System,
    system_manager::SystemManager,
};
use parking_lot::Mutex;

static PROCESSOR: LazyLock<Mutex<Processor>> = LazyLock::new(|| Mutex::new(Processor::new()));

fn update_sample_system(
    coor: &mut Coordinator,
    em: &mut EntityManager,
    cm: &mut ComponentManager,
    sm: &mut SystemManager,
    entities: &HashSet<Entity>,
    delta: f64,
) {
    //let container = Container::new();
    //let component_manager = container.get_service::<ComponentManager>();
    //let mut cm = component_manager.lock();

    //let count = entities.len();
    //log(format!("sample system updating {count}"));

    for entity in entities {
        let body = cm.get_component::<RigidBody>(*entity);
        let transform = cm.get_component::<Transform>(*entity);
        let gravity = cm.get_component::<Gravity>(*entity);
        
        body.velocity += *gravity.direction * delta;
        

        
        transform[0].position += body[0].velocity * delta;

        if transform[0].position.y < 0.0 {
            coor.destroy_entity(*entity);
            add_entity(coor, em);
        }
    }
}

fn register_sample_system(cm: &mut ComponentManager, sm: &mut SystemManager) {
    let mut signature = Signature::zero();
    signature.set(cm.get_component_type::<Transform>());
    signature.set(cm.get_component_type::<Gravity>());
    signature.set(cm.get_component_type::<RigidBody>());
    let sample_system = System::new(signature, update_sample_system);

    sm.register_system(sample_system);
}

fn add_entity(coor: &mut Coordinator, em: &mut EntityManager) {
    let y = 100.0;

    let entity = em.create_entity();
    coor.add_component(entity, Transform::new(0.0, y, 0.0));
    coor.add_component(entity, Gravity::new());
    coor.add_component(entity, RigidBody::new());
}

#[unsafe(no_mangle)]
pub extern "C" fn actr_init(_w: f32, _h: f32) {
    panic::set_hook(Box::new(|info| {
        log(info.to_string());
    }));
    let processor = PROCESSOR.lock();
    log(format!("init"));

    processor.send_message(ecs::message::Message::CoorECS(Box::new(
        |coor: &mut Coordinator,
         em: &mut EntityManager,
         cm: &mut ComponentManager,
         sm: &mut SystemManager| {
            cm.register_component::<Transform>(Transform::new(0.0, 0.0, 0.0));
            cm.register_component::<Gravity>(Gravity::zero());
            cm.register_component::<RigidBody>(RigidBody::new());

            register_sample_system(cm, sm);

            for n in 0..4096 {
                //log(format!("new entity {n}"));
                add_entity(coor, em);
            }
        },
    )));
}

#[unsafe(no_mangle)]
pub extern "C" fn actr_step(delta: f64) {
    //let mut container = Container::new();

    ///let mut coordinator = container.get_service::<Coordinator>();
    //let mut coor = coordinator.lock();
    let start = unsafe { actr_performance() };

    //coor.update(delta);

    let end = unsafe { actr_performance() };

    let time = end - start;

    log(format!("time {time}"));
}

pub fn log(message: String) {
    unsafe {
        _actr_log_length(message.as_ptr(), message.len());
    }
}
