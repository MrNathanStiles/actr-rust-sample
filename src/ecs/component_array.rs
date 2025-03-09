use std::{alloc::{self, Layout}, collections::HashMap};
use std::ptr;

use super::{Entity, MAX_ENTITIES};


pub struct ComponentArray<T> {
    pointer: *mut T,
    size: isize,
    entity_to_index: HashMap<Entity, isize>,
    index_to_entity: HashMap<isize, Entity>,
}

impl<T> ComponentArray<T> {

    pub fn new() -> ComponentArray<T>
    where
        T: 'static
    {
        let layout = Layout::array::<T>(MAX_ENTITIES).unwrap();
        ComponentArray {
            pointer: unsafe { alloc::alloc_zeroed(layout) as *mut T},
            size: 0,
            entity_to_index: HashMap::new(),
            index_to_entity: HashMap::new()
        }
    }

    pub fn insert_data(&mut self, entity: Entity, cmp: &mut T) {
        let index = self.size;
        self.entity_to_index.insert(entity, index);
        self.index_to_entity.insert(index, entity);
        let mut _existing = unsafe{ &mut * self.pointer.offset(index) };
        _existing = cmp;
    }

    pub fn remove_data(&mut self, entity: Entity) {
        let index_removed = *self.entity_to_index.get(&entity).unwrap();
        let index_last = self.size - 1;
        
        let entity_last = *self.index_to_entity.get(&index_last).unwrap();
        
        self.entity_to_index.insert(entity_last, index_removed);
        self.index_to_entity.insert(index_removed, entity_last);
        
        unsafe {
            let src = self.pointer.offset(index_removed);
            let dst = self.pointer.offset(index_last);
            ptr::copy_nonoverlapping(src, dst, 1);
        }

        self.size -= 1;
    }


    pub fn get_component(&self, entity: Entity) -> &mut T {
        let index = self.entity_to_index.get(&entity).unwrap();
        unsafe {&mut *self.pointer.offset(*index) }
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        if self.entity_to_index.contains_key(&entity) {
            self.remove_data(entity);
        }
    }

}
