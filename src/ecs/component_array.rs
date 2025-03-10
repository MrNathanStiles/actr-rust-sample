use std::{alloc::{self, Layout}, collections::HashMap};
use std::ptr;

use super::{Entity, MAX_ENTITIES};

pub struct ComponentArray {
    generic_pointer: *mut u8,
    entity_to_index: HashMap<Entity, usize>,
    index_to_entity: HashMap<usize, Entity>,
    component_count: usize,
    component_size: usize,
}

impl ComponentArray {
    pub fn new<T>() -> ComponentArray
    where 
        T: 'static
    {

        let layout = Layout::array::<T>(MAX_ENTITIES).unwrap();
        let generic_pointer = unsafe { alloc::alloc_zeroed(layout) };
        
        ComponentArray {
            generic_pointer,
            entity_to_index: HashMap::new(),
            index_to_entity: HashMap::new(),
            component_count: 0,
            component_size: ComponentArray::get_component_size::<T>(generic_pointer),
        }
    }

    fn get_component_size<T>(generic_pointer: *mut u8) -> usize {
        let pointer = generic_pointer as *mut T;
        let start = pointer.addr();
        let next = unsafe { pointer.offset(1).addr() };
        next - start
    }

    pub fn entity_destroyed(&mut self, entity: Entity)
    {
        if self.entity_to_index.contains_key(&entity) {
            self.remove_data(entity);
        }
    }

    pub fn remove_data(&mut self, entity: Entity) {
        let index_removed = *self.entity_to_index.get(&entity).unwrap();
        let index_last = self.component_count - 1;
        
        let entity_last = *self.index_to_entity.get(&index_last).unwrap();
        
        self.entity_to_index.insert(entity_last, index_removed);
        self.index_to_entity.insert(index_removed, entity_last);
    
        unsafe {
            let src = self.generic_pointer.add(index_removed * self.component_size);
            let dst = self.generic_pointer.add(index_last * self.component_size);
            ptr::copy_nonoverlapping(src, dst, 1);
        }

        self.component_count -= 1;
    }

    pub fn insert_data<T>(&mut self, entity: Entity, mut cmp: T)
    where
        T: 'static
    {
        let index = self.component_count;
        self.entity_to_index.insert(entity, index);
        self.index_to_entity.insert(index, entity);

        let mut _existing = unsafe{ self.generic_pointer.add(index) as *mut T};
        _existing = &mut cmp;
        self.component_count += 1;
    }   


    pub fn get_component<T>(&self, entity: Entity) -> &mut T 
    where 
        T: 'static
    {
        let index = self.entity_to_index.get(&entity).unwrap();
        let pointer = self.generic_pointer as *mut T;
        let pointer = unsafe { pointer.add(*index) };
        unsafe { &mut *pointer.add(*index) }
    }

}
