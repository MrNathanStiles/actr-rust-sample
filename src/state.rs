use crate::ecs::coordinator::Coordinator;

pub struct State {
    pub ecs: Coordinator,
}

impl State {
    pub fn new() -> State {
        State {
            ecs: Coordinator::new(),
        }
    }
}
