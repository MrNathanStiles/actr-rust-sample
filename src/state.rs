
use crate::component::Component;

pub struct State {
    pub component: Component
}

impl State {
    pub fn new() -> State {
        State {
            component: Component::new()
        }
    }
}
