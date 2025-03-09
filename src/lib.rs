mod actr;
mod component;
mod ecs;
mod state;

use std::collections::HashSet;

use actr::{_actr_log_length, actr_performance};
use component::transform::Transform;
use ecs::{coordinator::Coordinator, signature::Signature, system::System, Entity, MAX_ENTITIES};
use state::State;

#[unsafe(no_mangle)]
pub extern fn actr_construct() -> *mut State {
    Box::into_raw(Box::new(State::new()))
}

fn update_sample_system(ecs: &mut Coordinator, entities: &HashSet<Entity>, delta: f64) {
    for entity in entities.iter() {
        let cmp = ecs.get_component::<Transform>(*entity);
        cmp.position.x += 1.0 * delta;
        let x = cmp.position.x;
        log(format!("update sample system {x}"));
    }
}

fn register_sample_system(state: &mut State) {
    let mut signature = Signature::zero();
    signature.set(state.ecs.get_component_type::<Transform>());
    let sample_system = System::new(signature, update_sample_system);
    state.ecs.register_system(sample_system);
}

#[unsafe(no_mangle)]
pub extern fn actr_init(state_pointer: *mut State, _w: f32, _h: f32) {
    
    //let state = unsafe { state_pointer.as_mut().unwrap() };
    let state = unsafe { state_pointer.as_mut().unwrap() };

    state.ecs.register_component::<Transform>();

    register_sample_system(state);
    

    for n in 0..10 {
        let mut t = Transform::new();
        state.ecs.add_component(n, &mut t);
    }
    //let state = unsafe { &mut((((*state_pointer)))) };
    //state.component_manager.register_component::<Transform>();
    //let x = state.component.count();
    //log(format!("init {x}"));

    //let q = ComponentArray;
}

#[unsafe(no_mangle)]
pub extern fn actr_step(state_pointer: *mut State, _delta: f32) {
    let state = unsafe { state_pointer.as_mut().unwrap() };

    let start = unsafe { actr_performance() };

    for n in 0..MAX_ENTITIES {
        //let t = state.component.get::<Transform>(n);

        //t.position.x += 0.02;
        //t.position.z = n as f64;
        
        //let x = t.position.x;
        //let z = t.position.z;
        //let x = state.component.count();
        //log(format!("0 step {x},{z}"));
    }

    let end = unsafe { actr_performance() };

    let time = end - start;

    log(format!("time {time}"));

    
}

fn log(message: String) {
    unsafe {
        _actr_log_length(message.as_ptr(), message.len());
    }
}


