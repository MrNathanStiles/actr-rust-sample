pub mod transform;

use std::{
    any::{Any, TypeId},
    collections::HashMap
};

const ENTITY_COUNT: usize = 1024;
pub struct Component {
    component_arrays: HashMap<TypeId, Box<dyn Any + 'static>>,
}



impl Component {
    pub fn new() -> Component {
        Component {
            component_arrays: HashMap::new()
        }
    }
    pub fn register<T>(&mut self, initial: T)
    where
        T: Copy + 'static,
    {
        let id = TypeId::of::<T>();
        let thing = Box::new(vec![initial; ENTITY_COUNT]);
        self.component_arrays.insert(id, thing);
    }

    pub fn count(&self) -> usize {
        self.component_arrays.len()
    }

    pub fn get<T>(self, index: usize) -> T
    where
        T: Copy + 'static,
    {
        let id = &TypeId::of::<T>();
        let bx= self.component_arrays.get(id).unwrap();
        let vec = bx.downcast_ref::<Vec<T>>().unwrap();
        vec[index]
    }

}
