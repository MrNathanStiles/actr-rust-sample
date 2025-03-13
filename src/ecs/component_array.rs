use std::ptr;
use std::{
    alloc::{self, Layout},
    collections::HashMap,
};
use super::{Entity, MAX_ENTITIES};

pub struct ComponentArray {
    pub generic_pointer: *mut u8,
    entity_to_index: HashMap<Entity, usize>,
    index_to_entity: HashMap<usize, Entity>,
    component_count: usize,
    component_size: usize,
}

impl ComponentArray {
    pub fn new<T>() -> ComponentArray
    where
        T: 'static,
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

    pub fn entity_destroyed(&mut self, entity: Entity) {
        if self.entity_to_index.contains_key(&entity) {
            self.remove_data(entity);
        }
    }
    
    pub fn remove_data(&mut self, entity: Entity) {
        let index_removed = *self.entity_to_index.get(&entity).unwrap();
        
        self.component_count -= 1;
        let index_last = self.component_count;

        if index_removed == index_last {
            return;
        }

        let entity_last = *self.index_to_entity.get(&index_last).unwrap();

        self.entity_to_index.insert(entity_last, index_removed);
        self.index_to_entity.insert(index_removed, entity_last);

        unsafe {
            let src = self
                .generic_pointer
                .add(index_last * self.component_size);
            let dst = self.generic_pointer.add(index_removed * self.component_size);
            ptr::copy_nonoverlapping(src, dst, self.component_size);
        }

        
    }

    pub fn insert_data<T>(&mut self, entity: Entity, cmp: T)
    where
        T: 'static,
    {
        let index = self.component_count;
        self.entity_to_index.insert(entity, index);
        self.index_to_entity.insert(index, entity);

        // self.log(format!("component insert entity {entity} index {index}"));

        unsafe {
            let pointer = self.generic_pointer as *mut T;
            let mut _existing = pointer.add(index);
            *_existing = cmp;
        }
        
        self.component_count += 1;
    }

    pub fn get_component<T>(&self, entity: Entity) -> &mut T
    where
        T: 'static,
    {
        let index = self.entity_to_index.get(&entity).unwrap();
        let pentity = self.index_to_entity.get(index).unwrap();
        if *pentity != entity {
            panic!("entity {entity} index {index} pentity {pentity}")
        }
        let pointer = self.generic_pointer as *mut T;
        //log(format!("entity {entity} index {index} pentity {pentity}"));
            
        unsafe {
            let pointer = pointer.add(*index);
            &mut *pointer
        }
    }
}
