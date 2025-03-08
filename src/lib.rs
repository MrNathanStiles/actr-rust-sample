mod actr;
mod component;
mod data;
mod state;

use std::alloc::{self, Layout};

use actr::_actr_log_length;
use component::{transform::Transform, ENTITY_COUNT};
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
    state.component.register::<Transform>();
    let x = state.component.count();
    log(format!("init {x}"));

}

#[unsafe(no_mangle)]
pub extern fn actr_step(state_pointer: *mut State, _delta: f32) {
    let state = unsafe { state_pointer.as_mut().unwrap() };
    
    let t = state.component.get::<Transform>(0);

    t.position.x += 0.01;
    t.position.z = 1.0;
    
    let x = t.position.x;
    let z = t.position.z;
    //let x = state.component.count();
    log(format!("0 step {x},{z}"));


    let t = state.component.get::<Transform>(1);

    t.position.x += 0.02;
    t.position.z = 2.0;
    
    let x = t.position.x;
    let z = t.position.z;
    //let x = state.component.count();
    log(format!("0 step {x},{z}"));
}

fn log(message: String) {
    unsafe {
        _actr_log_length(message.as_ptr(), message.len());
    }
}


