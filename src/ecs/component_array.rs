use std::{alloc::{self, Layout}, collections::HashMap, fmt::Pointer};
use std::ptr;

use super::{Entity, MAX_ENTITIES};

pub struct ComponentArrayGeneric {
    pointer: *mut u8,
    entity_to_index: HashMap<Entity, usize>,
    index_to_entity: HashMap<usize, Entity>,
    component_size: usize,
    component_count: usize
}

impl ComponentArrayGeneric {
    pub fn new(pointer: *mut u8, component_size: usize) -> ComponentArrayGeneric {
        ComponentArrayGeneric {
            pointer,
            entity_to_index: HashMap::new(),
            index_to_entity: HashMap::new(),
            component_size,
            component_count: 0
        }
    }
    pub fn entity_destroyed(&mut self, entity: Entity) {
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
            let src = self.pointer.add(index_removed * self.component_size);
            let dst = self.pointer.add(index_last * self.component_size);
            ptr::copy_nonoverlapping(src, dst, 1);
        }

        self.component_count -= 1;
    }
}

pub struct ComponentArray<T> {
    pub generic_array: ComponentArrayGeneric,
    pointer: *mut T
}

impl<T> ComponentArray<T> {
    pub fn new() -> ComponentArray<T>
    where
        T: 'static
    {
        let layout = Layout::array::<T>(MAX_ENTITIES).unwrap();
        let generic_pointer = unsafe { alloc::alloc_zeroed(layout) };
        let pointer = generic_pointer as *mut T;
        let start = pointer.addr();
        let next = unsafe { pointer.offset(1).addr() };
        let size = next - start;
        let generic_array = ComponentArrayGeneric::new(generic_pointer, size);

        ComponentArray {
            generic_array,
            pointer,
        }
    }

    pub fn insert_data(&mut self, entity: Entity, cmp: &mut T) {
        let index = self.generic_array.component_count;
        self.generic_array.entity_to_index.insert(entity, index);
        self.generic_array.index_to_entity.insert(index, entity);
        let mut _existing = unsafe{ &mut * self.pointer.add(index) };
        _existing = cmp;
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        self.generic_array.entity_destroyed(entity);
    }

    pub fn remove_data(&mut self, entity: Entity) {
        self.generic_array.remove_data(entity);
    }


    pub fn get_component(&self, entity: Entity) -> &mut T {
        let index = self.generic_array.entity_to_index.get(&entity).unwrap();
        unsafe {&mut *self.pointer.add(*index) }
    }

    

}
