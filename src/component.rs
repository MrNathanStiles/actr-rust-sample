pub mod transform;

use std::{
    alloc::{self, Layout}, any::TypeId, collections::HashMap
};

pub const ENTITY_COUNT: usize = 1024;
pub struct Component {
    component_arrays: HashMap<TypeId, *mut u8>,
}

impl Component {
    pub fn new() -> Component {
        Component {
            component_arrays: HashMap::new(),
        }
    }
    pub fn count(&self) -> usize {
        self.component_arrays.len()
    }

    pub fn register<T>(&mut self)
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();
        let layout = Layout::array::<T>(ENTITY_COUNT).unwrap();
        let pointer = unsafe { alloc::alloc_zeroed(layout) };
        self.component_arrays.insert(id, pointer);
    }

    pub fn get<T>(&self, index: usize) -> &mut T
    where
        T: 'static,
    {
        let id = &TypeId::of::<T>();
        let pointer = self.component_arrays.get(id).unwrap();
        let loc = unsafe { pointer.add(index * size_of::<T>()) };
        let t = loc as *mut T;
        unsafe{&mut * t}
    }
}
