use std::{any::Any, collections::HashMap};

use as_any::{AsAny, Downcast};

use crate::actr::_actr_log_length;

use super::{Entity, MAX_ENTITIES};

trait QueryData: Sized {
    unsafe fn get(world: &World) -> Option<Self>,
}

impl<C: Component> QueryData for &C {
    unsafe fn get(world: &World) -> Option<Self> {
        ... // fun stuff!
    }
}

impl<Q1: QueryData, Q2: QueryData> QueryData for (Q1, Q2) {
    unsafe fn get(world: &World) -> Option<Self> {
        (Q1::get(world), Q2::get(world))
    }
}

pub struct ComponentArray<T: 'static>
{
    components: Vec<T>,
    entity_to_index: HashMap<Entity, usize>,
    index_to_entity: HashMap<usize, Entity>,
    component_count: usize,
}

pub trait ThingTrait: Any + AsAny + Send
{
    fn entity_destroyed(&mut self, entity: Entity);
    fn remove_data(&mut self, entity: Entity);
}

impl<T: 'static> ThingTrait for ComponentArray<T>
where 
    T: 'static + Send + Clone
{
    fn entity_destroyed(&mut self, entity: Entity)
    where 
        T: 'static + Clone
    {
        if self.entity_to_index.contains_key(&entity) {
            self.remove_data(entity);
        }
    }

    fn remove_data(&mut self, entity: Entity)
    where 
        T: 'static + Clone
    {

        let index_removed = *self.entity_to_index.get(&entity).unwrap();

        self.component_count -= 1;
        let index_last = self.component_count;

        if index_removed == index_last {
            return;
        }

        let entity_last = *self.index_to_entity.get(&index_last).unwrap();

        self.entity_to_index.insert(entity_last, index_removed);
        self.index_to_entity.insert(index_removed, entity_last);

        

        self.components[index_removed] = self.components[index_last].clone();
    }

}

impl<T> ComponentArray<T>
{
    pub fn new(default: T) -> ComponentArray<T>
    where
        T: Copy + Send + 'static,
    {
        let mut components = Vec::<T>::with_capacity(MAX_ENTITIES);

        for n in 0..MAX_ENTITIES {
            components.push(default);
        }
        ComponentArray {
            components,
            entity_to_index: HashMap::new(),
            index_to_entity: HashMap::new(),
            component_count: 0,
        }
    }

    pub fn log(message: String) {
        unsafe {
            //_actr_log_length(message.as_ptr(), message.len());
        }
    }
    

    pub fn insert_data(&mut self, entity: Entity, cmp: T)
    {

        
        let index = self.component_count;
        self.entity_to_index.insert(entity, index);
        self.index_to_entity.insert(index, entity);

        ComponentArray::<T>::log(format!("set hash maps, index {index}"));

        self.components[index] = cmp;
        ComponentArray::<T>::log(format!("assigned value"));

        self.component_count += 1;
        ComponentArray::<T>::log(format!("component count incremented"));

    }

    fn to_generic_pointer<T>(thing: T) -> usize
    where
        T: 'static,
    {
        let bx = Box::new(thing);
        Box::into_raw(bx) as usize
    }

    fn from_generic_pointer<T>(generic_pointer: usize) -> &'static mut T
    where
        T: 'static,
    {
        unsafe { &mut *(*(generic_pointer as *mut u8) as *mut T) }
    }

    pub fn get_component(&self, entity: Entity) -> &mut T
    where
        T: 'static,
    {
        let index = *self.entity_to_index.get(&entity).unwrap();
        
        let cmp = self.components[index];

        let s = &mut unsafe {
            // Copy whatever is stored at the raw pointer to a new owned value
            *self.components[0].as_ptr()
        };
        
        let t = unsafe {
            // Create a new mutable reference to a raw pointer
            &mut *self.v[0].as_ptr()
        };

        let start =  self.components.as_ptr();

        let foo = unsafe { &mut *self.components[*index].as_ptr };
        foo
        
        //let mut bar = foo.nth(*index).unwrap();
        //let baz = foo.as_any_mut();
        //let clu = baz.downcast_mut::<T>().unwrap();
        //clu
        //let pentity = self.index_to_entity.get(index).unwrap();
        //if *pentity != entity {
            //panic!("entity {entity} index {index} pentity {pentity}")
        //}
        //self.components.get_mut(*index).unwrap()
        
    }




}
