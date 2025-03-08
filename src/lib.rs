mod actr;
mod component;
mod data;
mod state;

use actr::_actr_log_length;
use component::transform::Transform;
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
    state.component.register(Transform::new());
    let x = state.component.count();
    log(format!("init {x}"));
}

#[unsafe(no_mangle)]
pub extern fn actr_step(state_pointer: *mut State, _delta: f32) {
    
    //let state = unsafe { state_pointer.as_ref().unwrap() };
    let state = unsafe { state_pointer.as_ref().unwrap() };
    //let state = unsafe { &(*state_pointer) };
    let x = state.component.count();
    log(format!("step {x}"));
}

fn log(message: String) {
    unsafe {
        _actr_log_length(message.as_ptr(), message.len());
    }
}


