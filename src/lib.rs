mod actr;
mod component;
mod ecs;
mod state;

use std::mem::take;

use actr::{_actr_log_length, actr_performance};

use component::vector3::Vector3;
use ecs::{system::System, MAX_ENTITIES};
use state::State;

#[unsafe(no_mangle)]
pub extern fn actr_construct() -> *mut State {
    Box::into_raw(Box::new(State::new()))
}

#[unsafe(no_mangle)]
pub extern fn actr_init(state_pointer: *mut State, _w: f32, _h: f32) {
    
    //let state = unsafe { state_pointer.as_mut().unwrap() };
    let state = unsafe { state_pointer.as_mut().unwrap() };
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


