use super::{Entity, component_array::ComponentArray};
use std::{any::TypeId, collections::HashMap};

pub type ComponentType = u64;
// pub const MAX_COMPONENTS: ComponentType = 64;

pub struct ComponentManager {
    component_array: HashMap<TypeId, ComponentArray>,
    component_types: HashMap<TypeId, ComponentType>,
    next_component_type: ComponentType,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            component_array: HashMap::new(),
            component_types: HashMap::new(),
            next_component_type: 1,
        }
    }

    pub fn register_component<T>(&mut self) -> usize
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();
        let array = ComponentArray::new::<T>();
        let address = array.generic_pointer as usize;
        self.component_array.insert(id, array);
        self.component_types.insert(id, self.next_component_type);
        self.next_component_type += 1;
        address
    }

    pub fn get_component_type<T>(&self) -> ComponentType
    where
        T: 'static,
    {
        let id = &TypeId::of::<T>();
        *self.component_types.get(id).unwrap()
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: T)
    where
        T: 'static,
    {
        let id = &TypeId::of::<T>();
        let result = self.component_array.get_mut(id).unwrap();
        result.insert_data(entity, component);
    }

    pub fn remove_component<T>(&mut self, entity: Entity)
    where
        T: 'static,
    {
        let id = &TypeId::of::<T>();
        let result = self.component_array.get_mut(id).unwrap();
        result.remove_data(entity);
    }

    pub fn get_component<T>(&self, entity: Entity) -> &mut T
    where
        T: 'static,
    {
        let id = &TypeId::of::<T>();
        let result = self.component_array.get(id).unwrap();
        result.get_component::<T>(entity)
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        for (_id, component) in self.component_array.iter_mut() {
            component.entity_destroyed(entity);
        }
    }
}
