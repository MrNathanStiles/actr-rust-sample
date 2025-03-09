use std::{ any::TypeId, collections::HashMap, ptr };
use super::{component_array::{ComponentArray, ComponentArrayGeneric}, Entity};

pub type ComponentType = u8;
pub const MAX_COMPONENTS: ComponentType = 64;

pub struct ComponentManager {
    component_array: HashMap<TypeId, *mut u8>,
    component_array_generic: HashMap<TypeId, ComponentArrayGeneric>,
    component_types: HashMap<TypeId, ComponentType>,
    next_component_type: ComponentType,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            component_array: HashMap::new(),
            component_array_generic: HashMap::new(),
            component_types: HashMap::new(),
            next_component_type: 0,
        }
    }
    pub fn count(&self) -> usize {
        self.component_array.len()
    }

    pub fn register_component<T>(&mut self, mut array: ComponentArray<T>)
    where
        T: 'static
    {
        let ptr = ptr::from_mut(&mut array) as *mut u8;
        let id = TypeId::of::<T>();
        // let ptr = &array as *mut u8;
        //let layout = Layout::array::<T>(MAX_ENTITIES).unwrap();
        //let pointer = unsafe { alloc::alloc_zeroed(layout) };
        self.component_array.insert(id, ptr);
        self.component_array_generic.insert(id, array.generic_array);
        self.component_types.insert(id, self.next_component_type);
        self.next_component_type += 1;
    }

    pub fn get_component_type<T>(&self) -> ComponentType
    where 
        T: 'static
    {
        let id = &TypeId::of::<T>();
        *self.component_types.get(id).unwrap()
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: &mut T) 
    where 
        T: 'static
    {
        let ar = self.get_component_array::<T>();
        ar.insert_data(entity, component);
    }

    pub fn remove_component<T>(&mut self, entity: Entity)
    where
        T: 'static
    {
        self.get_component_array::<T>().remove_data(entity);
    }

    pub fn get_component<T>(&self, entity: Entity) -> &mut T
    where
        T: 'static
    {
        self.get_component_array().get_component(entity)
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        for (id, system) in self.component_array_generic.iter_mut() {
            system.remove_data(entity);
        }
    }

    fn get_component_array<T>(&self) -> &mut ComponentArray<T>
    where
        T: 'static
    {
        let id = &TypeId::of::<T>();
        let result = self.component_array.get(id).unwrap();
        let result = result.cast::<ComponentArray<T>>();
        unsafe { &mut *result }
    }

    pub fn zget<T>(&self, index: usize)
    where
        T: 'static
    {
        let id = &TypeId::of::<T>();
        let result = self.component_array.get(id).unwrap();
        let result = *result;
        let pointer = result as *mut ComponentArray<T>;
        //let loc = unsafe { *pointer.add(index) };
        // loc.get_component(entity)
    }
}
